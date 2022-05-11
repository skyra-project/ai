use crate::common::players::{Players, INVALID_INDEX};
use std::cmp;

pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;
pub const BOARD_CELLS: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub type AiCells = [Players; BOARD_CELLS];
pub type AiRemaining = [u8; BOARD_WIDTH];

pub struct AiBoard {
	pub cells: AiCells,
	pub remaining: AiRemaining,
}

impl AiBoard {
	pub fn new(cells: AiCells) -> Self {
		let mut remaining = [0; BOARD_WIDTH];
		for x in 0..BOARD_WIDTH {
			let mut y: u8 = 0;
			while y < BOARD_HEIGHT as u8 && cells[y as usize * BOARD_WIDTH + x] == Players::Unset {
				y += 1;
			}

			remaining[x] = y;
		}

		Self { cells, remaining }
	}
}

const AVAILABLE_BOTTOM: [u8; BOARD_CELLS] = [
	0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, //
	0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, //
	0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, //
	0b0010, 0b0010, 0b0010, 0b0010, 0b0010, 0b0010, 0b0010, //
	0b0001, 0b0001, 0b0001, 0b0001, 0b0001, 0b0001, 0b0001, //
	0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
];

const AVAILABLE_HORIZONTAL: [u8; BOARD_CELLS] = [
	0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100, //
	0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100, //
	0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100, //
	0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100, //
	0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100, //
	0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100,
];

const AVAILABLE_DIAGONAL_TL: [u8; BOARD_CELLS] = [
	0b0011, 0b0011, 0b0011, 0b0011, 0b0010, 0b0001, 0b0000, //
	0b0011, 0b0111, 0b0111, 0b0111, 0b0110, 0b0101, 0b0100, //
	0b0011, 0b0111, 0b1011, 0b1011, 0b1010, 0b1001, 0b1000, //
	0b0010, 0b0110, 0b1010, 0b1110, 0b1110, 0b1101, 0b1100, //
	0b0001, 0b0101, 0b1001, 0b1101, 0b1101, 0b1101, 0b1100, //
	0b0000, 0b0100, 0b1000, 0b1100, 0b1100, 0b1100, 0b1100,
];

const AVAILABLE_DIAGONAL_BL: [u8; BOARD_CELLS] = [
	0b0000, 0b0100, 0b1000, 0b1100, 0b1100, 0b1100, 0b1100, //
	0b0001, 0b0101, 0b1001, 0b1101, 0b1101, 0b1101, 0b1100, //
	0b0010, 0b0110, 0b1010, 0b1110, 0b1110, 0b1101, 0b1100, //
	0b0011, 0b0111, 0b1011, 0b1011, 0b1010, 0b1001, 0b1000, //
	0b0011, 0b0111, 0b0111, 0b0111, 0b0110, 0b0101, 0b0100, //
	0b0011, 0b0011, 0b0011, 0b0011, 0b0010, 0b0001, 0b0000,
];

const UNDEFINED_LAST_MOVE: usize = usize::MAX;

struct AiResults {
	pub points: i8,
	pub position: usize,
}

fn compare(a: Players, b: Players, c: Players, d: Players) -> bool {
	a == b && b == c && c == d
}

impl AiBoard {
	unsafe fn check_4(self: &AiBoard, cell: usize, a: isize, b: isize, c: isize, d: isize) -> bool {
		let ptr = self.cells.as_ptr().add(cell);

		let ca = *ptr.offset(a);
		let cb = *ptr.offset(b);
		let cc = *ptr.offset(c);
		let cd = *ptr.offset(d);

		compare(ca, cb, cc, cd)
	}

	unsafe fn check_5(self: &AiBoard, cell: usize, a: isize, b: isize, c: isize, d: isize, e: isize) -> bool {
		let ptr = self.cells.as_ptr().add(cell);

		let ca = *ptr.offset(a);
		let cb = *ptr.offset(b);
		let cc = *ptr.offset(c);
		let cd = *ptr.offset(d);
		let ce = *ptr.offset(e);

		compare(ca, cb, cc, cd) || compare(cb, cc, cd, ce)
	}

	#[allow(clippy::too_many_arguments)]
	unsafe fn check_6(self: &AiBoard, cell: usize, a: isize, b: isize, c: isize, d: isize, e: isize, f: isize) -> bool {
		let ptr = self.cells.as_ptr().add(cell);

		let ca = *ptr.offset(a);
		let cb = *ptr.offset(b);
		let cc = *ptr.offset(c);
		let cd = *ptr.offset(d);
		let ce = *ptr.offset(e);
		let cf = *ptr.offset(f);

		compare(ca, cb, cc, cd) || compare(cb, cc, cd, ce) || compare(cc, cd, ce, cf)
	}

	#[allow(clippy::too_many_arguments)]
	unsafe fn check_7(
		self: &AiBoard,
		cell: usize,
		a: isize,
		b: isize,
		c: isize,
		d: isize,
		e: isize,
		f: isize,
		g: isize,
	) -> bool {
		let ptr = self.cells.as_ptr().add(cell);

		let ca = *ptr.offset(a);
		let cb = *ptr.offset(b);
		let cc = *ptr.offset(c);
		let cd = *ptr.offset(d);
		let ce = *ptr.offset(e);
		let cf = *ptr.offset(f);
		let cg = *ptr.offset(g);

		compare(ca, cb, cc, cd) || compare(cb, cc, cd, ce) || compare(cc, cd, ce, cf) || compare(cd, ce, cf, cg)
	}

	#[allow(clippy::too_many_arguments)]
	unsafe fn status_row(
		self: &AiBoard,
		mask: u8,
		cell: usize,
		l1: isize,
		l2: isize,
		l3: isize,
		r1: isize,
		r2: isize,
		r3: isize,
	) -> bool {
		match mask {
			// 0bxx11:
			0b0011 => self.check_4(cell, 0, r1, r2, r3),
			0b0111 => self.check_5(cell, l1, 0, r1, r2, r3),
			0b1011 => self.check_6(cell, l2, l1, 0, r1, r2, r3),
			// 0b11xx:
			0b1100 => self.check_4(cell, l3, l2, l1, 0),
			0b1101 => self.check_5(cell, l3, l2, l1, 0, r1),
			0b1110 => self.check_6(cell, l3, l2, l1, 0, r1, r2),
			0b1111 => self.check_7(cell, l3, l2, l1, 0, r1, r2, r3),

			// 0b01xx \ {0b0100, 0b0101} -> {0b0110, [0b0111]}:
			0b0110 => self.check_4(cell, l1, 0, r1, r2),
			// 0b10xx \ {0b1000} -> {0b1001, 0b1010, [0b1011]}:
			0b1001 => self.check_4(cell, l2, l1, 0, r1),
			0b1010 => self.check_5(cell, l2, l1, 0, r1, r2),
			// 0bxx01 \ {0b0001, 0b0101} -> {[0b1001], [0b1101]}:
			// 0bxx10 \ {0b0010} -> {[0b0110], [0b1010], [0b1100]}:

			// Handle all other cases:
			_ => false,
		}
	}

	unsafe fn status(self: &AiBoard, last_move: usize) -> bool {
		if last_move == UNDEFINED_LAST_MOVE {
			return false;
		}

		const I_BOARD_WIDTH: isize = BOARD_WIDTH as isize;

		// Vertical
		const B1: isize = I_BOARD_WIDTH;
		const B2: isize = B1 * 2;
		const B3: isize = B1 * 3;
		if AVAILABLE_BOTTOM[last_move] == 3 && self.check_4(last_move, 0, B1, B2, B3) {
			return true;
		}

		// Horizontal
		const L1: isize = -1;
		const L2: isize = L1 * 2;
		const L3: isize = L1 * 3;
		const R1: isize = 1;
		const R2: isize = R1 * 2;
		const R3: isize = R1 * 3;

		// Top Left to Bottom Right
		const TL1: isize = -I_BOARD_WIDTH - 1;
		const TL2: isize = TL1 * 2;
		const TL3: isize = TL1 * 3;
		const BR1: isize = I_BOARD_WIDTH + 1;
		const BR2: isize = BR1 * 2;
		const BR3: isize = BR1 * 3;

		// Bottom Left to Top Right
		const BL1: isize = I_BOARD_WIDTH - 1;
		const BL2: isize = BL1 * 2;
		const BL3: isize = BL1 * 3;
		const TR1: isize = -I_BOARD_WIDTH + 1;
		const TR2: isize = TR1 * 2;
		const TR3: isize = TR1 * 3;

		self.status_row(AVAILABLE_HORIZONTAL[last_move], last_move, L1, L2, L3, R1, R2, R3)
			|| self.status_row(AVAILABLE_DIAGONAL_TL[last_move], last_move, TL1, TL2, TL3, BR1, BR2, BR3)
			|| self.status_row(AVAILABLE_DIAGONAL_BL[last_move], last_move, BL1, BL2, BL3, TR1, TR2, TR3)
	}

	fn column_available(self: &AiBoard, column: usize) -> bool {
		self.remaining[column] != 0
	}

	fn piece_offset(self: &AiBoard, column: usize) -> usize {
		((self.remaining[column] as usize - 1) * BOARD_WIDTH) + column
	}

	fn piece_add(self: &mut AiBoard, column: usize, offset: usize, player: Players) {
		self.remaining[column] -= 1;
		self.cells[offset] = player;
	}

	fn piece_remove(self: &mut AiBoard, column: usize, offset: usize) {
		self.remaining[column] += 1;
		self.cells[offset] = Players::Unset;
	}

	/// Minimum is `Players::Player`
	fn min(self: &mut AiBoard, last_move: usize, remaining: u8, alpha: i8, beta: i8) -> AiResults {
		unsafe {
			if self.status(last_move) {
				return AiResults { points: 100, position: INVALID_INDEX };
			}
		}

		if remaining == 0 {
			return AiResults { points: 0, position: INVALID_INDEX };
		}

		// Possible values for min_v are:
		// -1 - win
		//  0 - a tie
		//  1 - loss
		//
		// We're initially setting it to 2 as worse than the worst case:
		let mut min_v = i8::MIN;
		let mut column = INVALID_INDEX;
		let mut local_beta = beta;

		for c in 0..BOARD_WIDTH {
			if self.column_available(c) {
				continue;
			}

			let offset = self.piece_offset(c);

			// On the empty field player Player makes a move and calls Max
			// That's one branch of the game tree:
			self.piece_add(c, offset, Players::Player);

			let m = self.min(offset, remaining - 1, alpha, local_beta).points;

			// Setting back the field to empty:
			self.piece_remove(c, offset);

			// Fixing the min_v value if needed:
			if m < min_v {
				min_v = m;
				column = c;

				local_beta = cmp::min(local_beta, min_v);
				if alpha >= local_beta {
					break;
				}
			}
		}

		AiResults { points: min_v, position: column }
	}

	/// Maximum is `Players::Machine`
	fn max(self: &mut AiBoard, last_move: usize, remaining: u8, alpha: i8, beta: i8) -> AiResults {
		unsafe {
			if self.status(last_move) {
				return AiResults { points: -100, position: INVALID_INDEX };
			}
		}

		if remaining == 0 {
			return AiResults { points: 0, position: INVALID_INDEX };
		}

		// Possible values for max_v are:
		// -1 - loss
		//  0 - a tie
		//  1 - win
		//
		// We're initially setting it to -2 as worse than the worst case:
		let mut max_v = i8::MIN;
		let mut column = INVALID_INDEX;
		let mut local_alpha = alpha;

		for c in 0..BOARD_WIDTH {
			if self.column_available(c) {
				continue;
			}

			let offset = self.piece_offset(c);

			// On the empty field player Machine makes a move and calls Min
			// That's one branch of the game tree:
			self.piece_add(c, offset, Players::Machine);

			let m = self.min(offset, remaining - 1, local_alpha, beta).points;

			// Setting back the field to empty:
			self.piece_remove(c, offset);

			// Fixing the max_v value if needed:
			if m > max_v {
				max_v = m;
				column = c;

				local_alpha = cmp::max(local_alpha, max_v);
				if local_alpha >= beta {
					break;
				}
			}
		}

		AiResults { points: max_v, position: column }
	}

	pub fn position(self: &mut AiBoard, remaining: u8, maximum_depth: u8) -> usize {
		// If remaining is 42, then the board is empty.
		//
		// Strategically speaking, the middle position in ConnectFour is always the best,
		// and very often a winner move. The algorithm will always pick this.
		//
		// We have this board:
		// 00 01 02 03 04 05 06
		// 07 08 09 10 11 12 13
		// 14 15 16 17 18 19 20
		// 21 22 23 24 25 26 27
		// 28 29 30 31 32 33 34
		// 35 36 37 38 39 40 41
		//
		// The center is 4, therefore, we return this number.
		//
		// Hardcoding this is useful, on an empty board, there are 4,531,985,219,092 possibilities.
		if remaining == 42 {
			return 3;
		}

		const DEFAULT_ALPHA: i8 = i8::MIN;
		const DEFAULT_BETA: i8 = i8::MAX;

		// Process the best move for the AI.
		self.max(UNDEFINED_LAST_MOVE, cmp::min(remaining, maximum_depth), DEFAULT_ALPHA, DEFAULT_BETA).position
	}
}
