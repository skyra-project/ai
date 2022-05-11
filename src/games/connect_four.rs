use crate::{Players, U_INVALID_INDEX};
use std::cmp;

pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;
pub const BOARD_CELLS: usize = BOARD_WIDTH * BOARD_HEIGHT;

const OUTCOME_HUMAN_WINS: i8 = -20;
const OUTCOME_MACHINE_WINS: i8 = 20;
const OUTCOME_DRAW: i8 = 0;

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

#[inline(always)]
fn compare(a: Players, b: Players, c: Players, d: Players) -> bool {
	a == b && b == c && c == d
}

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
			let mut y = 0;
			while y < BOARD_HEIGHT && cells[y * BOARD_WIDTH + x] == Players::Unset {
				y += 1;
			}

			remaining[x] = y as u8;
		}

		Self { cells, remaining }
	}

	unsafe fn check_4(self: &AiBoard, cell: usize, a: isize, b: isize, c: isize, d: isize) -> bool {
		debug_assert!(cell as isize + a >= 0);
		debug_assert!(cell as isize + d < BOARD_CELLS as isize);
		debug_assert!(self.cells[cell] != Players::Unset);

		let ptr = self.cells.as_ptr().add(cell);

		let ca = *ptr.offset(a);
		let cb = *ptr.offset(b);
		let cc = *ptr.offset(c);
		let cd = *ptr.offset(d);

		compare(ca, cb, cc, cd)
	}

	unsafe fn check_5(self: &AiBoard, cell: usize, a: isize, b: isize, c: isize, d: isize, e: isize) -> bool {
		debug_assert!(cell as isize + a >= 0);
		debug_assert!(cell as isize + e < BOARD_CELLS as isize);
		debug_assert!(self.cells[cell] != Players::Unset);

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
		debug_assert!(cell as isize + a >= 0);
		debug_assert!(cell as isize + f < BOARD_CELLS as isize);
		debug_assert!(self.cells[cell] != Players::Unset);

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
		debug_assert!(cell as isize + a >= 0);
		debug_assert!(cell as isize + g < BOARD_CELLS as isize);
		debug_assert!(self.cells[cell] != Players::Unset);

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

	unsafe fn status(self: &AiBoard, last_cell_offset: usize) -> bool {
		debug_assert!(last_cell_offset < BOARD_CELLS);
		debug_assert!(self.cells[last_cell_offset] != Players::Unset);

		const I_BOARD_WIDTH: isize = BOARD_WIDTH as isize;

		// Vertical
		const B1: isize = I_BOARD_WIDTH;
		const B2: isize = B1 * 2;
		const B3: isize = B1 * 3;
		if AVAILABLE_BOTTOM[last_cell_offset] == 3 && self.check_4(last_cell_offset, 0, B1, B2, B3) {
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

		self.status_row(AVAILABLE_HORIZONTAL[last_cell_offset], last_cell_offset, L1, L2, L3, R1, R2, R3)
			|| self.status_row(AVAILABLE_DIAGONAL_TL[last_cell_offset], last_cell_offset, TL1, TL2, TL3, BR1, BR2, BR3)
			|| self.status_row(AVAILABLE_DIAGONAL_BL[last_cell_offset], last_cell_offset, BL1, BL2, BL3, TR1, TR2, TR3)
	}

	fn column_available(self: &AiBoard, column: usize) -> bool {
		debug_assert!(column < BOARD_WIDTH);

		self.remaining[column] != 0
	}

	fn piece_offset(self: &AiBoard, column: usize) -> usize {
		debug_assert!(column < BOARD_WIDTH);
		debug_assert!(self.remaining[column] != 0);

		((self.remaining[column] as usize - 1) * BOARD_WIDTH) + column
	}

	fn piece_add(self: &mut AiBoard, column: usize, offset: usize, player: Players) {
		debug_assert!(column < BOARD_WIDTH);
		debug_assert!(self.remaining[column] != 0);
		debug_assert!(offset < BOARD_CELLS);
		debug_assert!(self.cells[offset] == Players::Unset);

		self.remaining[column] -= 1;
		self.cells[offset] = player;
	}

	fn piece_remove(self: &mut AiBoard, column: usize, offset: usize) {
		debug_assert!(column < BOARD_WIDTH);
		debug_assert!(self.remaining[column] != BOARD_HEIGHT as u8);
		debug_assert!(offset < BOARD_CELLS);
		debug_assert!(self.cells[offset] != Players::Unset);

		self.remaining[column] += 1;
		self.cells[offset] = Players::Unset;
	}

	/// Minimum is `Players::Player`
	fn min(self: &mut AiBoard, last_cell_offset: usize, remaining: u8, alpha: i8, beta: i8) -> i8 {
		unsafe {
			if self.status(last_cell_offset) {
				return OUTCOME_MACHINE_WINS;
			}
		}

		if remaining == 0 {
			return OUTCOME_DRAW;
		}

		// Possible values for min_v are:
		// -1 - win
		//  0 - a tie
		//  1 - loss
		//
		// We're initially setting it to 2 as worse than the worst case:
		let mut min_v = i8::MIN;
		let mut local_beta = beta;

		for c in 0..BOARD_WIDTH {
			if !self.column_available(c) {
				continue;
			}

			let offset = self.piece_offset(c);

			// On the empty field player Player makes a move and calls Max
			// That's one branch of the game tree:
			self.piece_add(c, offset, Players::Player);

			let m = self.max(offset, remaining - 1, alpha, local_beta);

			// Setting back the field to empty:
			self.piece_remove(c, offset);

			// Fixing the min_v value if needed:
			if m < min_v {
				min_v = m;

				local_beta = cmp::min(local_beta, min_v);
				if alpha >= local_beta {
					break;
				}
			}
		}

		min_v
	}

	/// Maximum is `Players::Machine`
	fn max(self: &mut AiBoard, last_cell_offset: usize, remaining: u8, alpha: i8, beta: i8) -> i8 {
		unsafe {
			if self.status(last_cell_offset) {
				return OUTCOME_HUMAN_WINS;
			}
		}

		if remaining == 0 {
			return OUTCOME_DRAW;
		}

		// Possible values for max_v are:
		// -1 - loss
		//  0 - a tie
		//  1 - win
		//
		// We're initially setting it to -2 as worse than the worst case:
		let mut max_v = i8::MIN;
		let mut local_alpha = alpha;

		for c in 0..BOARD_WIDTH {
			if !self.column_available(c) {
				continue;
			}

			let offset = self.piece_offset(c);

			// On the empty field player Machine makes a move and calls Min
			// That's one branch of the game tree:
			self.piece_add(c, offset, Players::Machine);

			let m = self.min(offset, remaining - 1, local_alpha, beta);

			// Setting back the field to empty:
			self.piece_remove(c, offset);

			// Fixing the max_v value if needed:
			if m > max_v {
				max_v = m;

				local_alpha = cmp::max(local_alpha, max_v);
				if local_alpha >= beta {
					break;
				}
			}
		}

		max_v
	}

	fn max_top(self: &mut AiBoard, remaining: u8) -> usize {
		if remaining == 0 {
			return U_INVALID_INDEX;
		}

		const DEFAULT_ALPHA: i8 = i8::MIN;
		const DEFAULT_BETA: i8 = i8::MAX;

		let mut max_v = i8::MIN;
		let mut column = U_INVALID_INDEX;
		for c in 0..BOARD_WIDTH {
			if !self.column_available(c) {
				continue;
			}

			let offset = self.piece_offset(c);

			// On the empty field player Machine makes a move and calls Min
			// That's one branch of the game tree:
			self.piece_add(c, offset, Players::Machine);

			let points = self.min(offset, remaining, DEFAULT_ALPHA, DEFAULT_BETA);

			// Setting back the field to empty:
			self.piece_remove(c, offset);

			if points > max_v {
				max_v = points;
				column = c;

				// Break the loop earlier if we have found a winning move:
				if points >= OUTCOME_MACHINE_WINS {
					break;
				}
			}
		}

		column
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
			3
		} else {
			// Process the best move for the AI.
			self.max_top(cmp::min(remaining, maximum_depth))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_compare_same() {
		assert!(compare(Players::Player, Players::Player, Players::Player, Players::Player));
	}

	macro_rules! compare_different {
		($($name:ident: $value:expr,)*) => {
		$(
			#[test]
			fn $name() {
				let (a, b, c, d) = $value;
				assert!(!compare(a, b, c, d));
			}
		)*
		}
	}

	compare_different! {
		test_compare_one_a_different: (Players::Machine, Players::Player, Players::Player, Players::Player),
		test_compare_one_b_different: (Players::Player, Players::Machine, Players::Player, Players::Player),
		test_compare_one_c_different: (Players::Player, Players::Player, Players::Machine, Players::Player),
		test_compare_one_d_different: (Players::Player, Players::Player, Players::Player, Players::Machine),
	}

	macro_rules! create_cells {
		() => ([Players::Unset; 42]);
		($($index:expr $(, $tail:expr)*)*) => ($({
			let mut tmp = create_cells!($($tail),*);
			tmp[$index] = Players::Player;
			tmp
		})*);
	}

	mod new {
		use super::*;

		#[test]
		fn test_empty() {
			let cells = create_cells!();
			let remaining: AiRemaining = [6; 7];
			let board = AiBoard::new(cells);

			assert_eq!(board.cells, cells);
			assert_eq!(board.remaining, remaining);
		}

		#[test]
		fn test_row_filled() {
			let cells = create_cells!(35, 36, 37, 38, 39, 40, 41);
			let remaining: AiRemaining = [5; 7];
			let board = AiBoard::new(cells);

			assert_eq!(board.cells, cells);
			assert_eq!(board.remaining, remaining);
		}

		#[test]
		fn test_column_filled() {
			let cells = create_cells!(0, 7, 14, 21, 28, 35);
			let remaining: AiRemaining = [0, 6, 6, 6, 6, 6, 6];
			let board = AiBoard::new(cells);

			assert_eq!(board.cells, cells);
			assert_eq!(board.remaining, remaining);
		}
	}

	mod check_4 {
		use super::super::*;

		macro_rules! test_panic {
			($($name:ident: [$cells:expr, $last_column:expr, $offsets:expr],)*) => ($(
				#[test]
				#[should_panic]
				fn $name() {
					let board = AiBoard::new($cells);

					unsafe {
						board.check_4($last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3);
					}
				}
			)*);
		}

		test_panic! {
			test_out_of_range_under: [create_cells!(0, 2, 3, 4), 0, (-1, 0, 1, 2)],
			test_out_of_range_over: [create_cells!(38, 39, 40, 41), 39, (0, 1, 2, 3)],
			test_invalid_offset: [create_cells!(0, 2), 1, (-1, 0, 1, 2)],
		}

		#[test]
		fn test_fails() {
			let board = AiBoard::new(create_cells!(0, 2, 3, 4));

			unsafe {
				assert!(!board.check_4(0, 0, 1, 2, 3));
			}
		}

		macro_rules! test_true {
			($($name:ident: [$last_column:expr, $offsets:expr],)*) => {
			$(
				#[test]
				fn $name() {
					let board = AiBoard::new(create_cells!(0, 1, 2, 3, 4));

					unsafe {
						assert!(board.check_4($last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3));
					}
				}
			)*
			};
		}

		test_true! {
			test_match_offset_0: [0, (0, 1, 2, 3)],
			test_match_offset_1: [1, (-1, 0, 1, 2)],
			test_match_offset_2: [2, (-2, -1, 0, 1)],
			test_match_offset_3: [3, (-3, -2, -1, 0)],
		}
	}

	mod check_5 {
		use super::super::*;

		macro_rules! test_panic {
			($($name:ident: [$cells:expr, $last_column:expr, $offsets:expr],)*) => ($(
				#[test]
				#[should_panic]
				fn $name() {
					let board = AiBoard::new($cells);

					unsafe {
						board.check_5($last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3, $offsets.4);
					}
				}
			)*);
		}

		test_panic! {
			test_out_of_range_under: [create_cells!(0, 2, 3, 4), 0, (-1, 0, 1, 2, 3)],
			test_out_of_range_over: [create_cells!(38, 39, 40, 41), 39, (-1, 0, 1, 2, 3)],
			test_invalid_offset: [create_cells!(0, 2, 3, 4, 5), 1, (-1, 0, 1, 2, 3)],
		}

		#[test]
		fn test_fails() {
			let board = AiBoard::new(create_cells!(0, 2, 3, 4));

			unsafe {
				assert!(!board.check_5(2, -1, 0, 1, 2, 3));
			}
		}

		macro_rules! test_true {
			($($name:ident: [$last_column:expr, $offsets:expr],)*) => {
			$(
				#[test]
				fn $name() {
					let board = AiBoard::new(create_cells!(0, 1, 2, 3));

					unsafe {
						assert!(board.check_5($last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3, $offsets.4));
					}
				}
			)*
			}
		}

		test_true! {
			// test_match_offset_0 is omitted because horizontal offset would go beyond [0, 3]
			test_match_offset_1: [1, (-1, 0, 1, 2, 3)],
			test_match_offset_2: [2, (-2, -1, 0, 1, 2)],
			test_match_offset_3: [3, (-3, -2, -1, 0, 1)],
		}
	}

	mod check_6 {
		use super::super::*;

		macro_rules! test_panic {
			($($name:ident: [$cells:expr, $last_column:expr, $offsets:expr],)*) => ($(
				#[test]
				#[should_panic]
				fn $name() {
					let board = AiBoard::new($cells);

					unsafe {
						board.check_6($last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3, $offsets.4, $offsets.5);
					}
				}
			)*);
		}

		test_panic! {
			test_out_of_range_under: [create_cells!(0, 2, 3, 4), 0, (-2, -1, 0, 1, 2, 3)],
			test_out_of_range_over: [create_cells!(38, 39, 40, 41), 39, (-2, -1, 0, 1, 2, 3)],
			test_invalid_offset: [create_cells!(0, 2, 3, 4, 5), 1, (-2, -1, 0, 1, 2, 3)],
		}

		#[test]
		fn test_fails() {
			let board = AiBoard::new(create_cells!(0, 2, 3, 4));

			unsafe {
				assert!(!board.check_6(2, -2, -1, 0, 1, 2, 3));
			}
		}

		macro_rules! test_true {
			($($name:ident: [$last_column:expr, $offsets:expr],)*) => {
			$(
				#[test]
				fn $name() {
					let board = AiBoard::new(create_cells!(0, 1, 2, 3, 4));

					unsafe {
						assert!(board.check_6($last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3, $offsets.4, $offsets.5));
					}
				}
			)*
			}
		}

		test_true! {
			// test_match_offset_0-1 is omitted because horizontal offset would go beyond [0, 3]
			test_match_offset_2: [2, (-2, -1, 0, 1, 2, 3)],
			test_match_offset_3: [3, (-3, -2, -1, 0, 1, 2)],
			test_match_offset_4: [4, (-3, -2, -1, 0, 1, 2)],
		}
	}

	mod check_7 {
		use super::super::*;

		macro_rules! test_panic {
			($($name:ident: [$cells:expr, $cell:expr],)*) => ($(
				#[test]
				#[should_panic]
				fn $name() {
					let board = AiBoard::new($cells);

					unsafe {
						board.check_7($cell, -3, -2, -1, 0, 1, 2, 3);
					}
				}
			)*);
		}

		test_panic! {
			test_out_of_range_under: [create_cells!(0, 2, 3, 4), 0],
			test_out_of_range_over: [create_cells!(38, 39, 40, 41), 39],
			test_invalid_offset: [create_cells!(0, 2, 3, 4, 5), 1],
		}

		#[test]
		fn test_fails() {
			let board = AiBoard::new(create_cells!(0, 2, 3, 4));

			unsafe {
				assert!(!board.check_7(3, -3, -2, -1, 0, 1, 2, 3));
			}
		}

		macro_rules! test_true {
			($($name:ident: [$last_column:expr],)*) => {
			$(
				#[test]
				fn $name() {
					let board = AiBoard::new(create_cells!(0, 1, 2, 3, 4));

					unsafe {
						assert!(board.check_7($last_column, -3, -2, -1, 0, 1, 2, 3));
					}
				}
			)*
			}
		}

		test_true! {
			// The only horizontal row with 3 cells in each side is the middle, 3
			test_match_offset_3: [3],
		}
	}

	mod status {
		use super::super::*;

		macro_rules! test_panic {
			($($name:ident: [$cells:expr, $last_cell_offset:expr],)*) => ($(
				#[test]
				#[should_panic]
				fn $name() {
					let board = AiBoard::new($cells);

					unsafe {
						board.status($last_cell_offset);
					}
				}
			)*);
		}

		test_panic! {
			test_out_of_range_over: [create_cells!(), 42],
			test_invalid_offset: [create_cells!(0, 2), 1],
		}

		macro_rules! test_false {
			($($name:ident: [$cells:expr, $last_cell_offset:expr],)*) => ($(
				#[test]
				fn $name() {
					let board = AiBoard::new($cells);

					unsafe {
						assert!(!board.status($last_cell_offset));
					}
				}
			)*);
		}

		test_false! {
			test_fail_horizontal_0: [create_cells!(0, 1, 3, 4), 0],
			test_fail_horizontal_1: [create_cells!(1, 2, 3), 1],
			test_fail_horizontal_2: [create_cells!(2), 2],
			test_fail_horizontal_3: [create_cells!(3), 3],
			test_fail_vertical_0: [create_cells!(21), 21],
			test_fail_vertical_1: [create_cells!(21, 28), 21],
			test_fail_vertical_2: [create_cells!(21, 28, 35), 28],
			test_fail_vertical_3: [create_cells!(37, 40), 40],
			test_fail_tl_br_0: [create_cells!(0, 8, 24), 0],
			test_fail_tl_br_1: [create_cells!(0, 8, 16), 8],
			test_fail_tl_br_2: [create_cells!(8, 16, 24), 16],
			test_fail_bl_tr_0: [create_cells!(21), 21],
			test_fail_bl_tr_1: [create_cells!(21, 15, 3), 21],
			test_fail_bl_tr_2: [create_cells!(21, 9, 3), 21],
		}

		macro_rules! test_true {
			($($name:ident: [$cells:expr, $last_cell_offset:expr],)*) => ($(
				#[test]
				fn $name() {
					let board = AiBoard::new($cells);

					unsafe {
						assert!(board.status($last_cell_offset));
					}
				}
			)*);
		}

		test_true! {
			test_pass_horizontal_offset_0: [create_cells!(0, 1, 2, 3), 0],
			test_pass_horizontal_offset_1: [create_cells!(0, 1, 2, 3), 1],
			test_pass_horizontal_offset_2: [create_cells!(0, 1, 2, 3), 2],
			test_pass_horizontal_offset_3: [create_cells!(0, 1, 2, 3), 3],
			test_pass_vertical_offset_0: [create_cells!(0, 7, 14, 21), 0],
			test_pass_vertical_offset_1: [create_cells!(7, 14, 21, 28), 7],
			test_pass_vertical_offset_2: [create_cells!(14, 21, 28, 35), 14],
			test_pass_tl_br_offset_0: [create_cells!(0, 8, 16, 24), 0],
			test_pass_tl_br_offset_1: [create_cells!(0, 8, 16, 24), 8],
			test_pass_tl_br_offset_2: [create_cells!(0, 8, 16, 24), 16],
			test_pass_tl_br_offset_3: [create_cells!(0, 8, 16, 24), 24],
			test_pass_bl_tr_offset_0: [create_cells!(21, 15, 9, 3), 21],
			test_pass_bl_tr_offset_1: [create_cells!(21, 15, 9, 3), 15],
			test_pass_bl_tr_offset_2: [create_cells!(21, 15, 9, 3), 9],
			test_pass_bl_tr_offset_3: [create_cells!(21, 15, 9, 3), 3],
		}
	}

	mod column_available {
		use super::super::*;

		#[test]
		#[should_panic]
		fn test_out_of_range_over() {
			let board = AiBoard::new(create_cells!());
			board.column_available(7);
		}

		#[test]
		fn test_0() {
			let board = AiBoard::new(create_cells!(0, 7, 14, 21, 28, 35));
			assert!(!board.column_available(0));
		}

		macro_rules! test_true {
			($($name:ident: $cells:expr,)*) => {
			$(
				#[test]
				fn $name() {
					let board = AiBoard::new($cells);
					assert!(board.column_available(0));
				}
			)*
			}
		}

		test_true! {
			test_1: create_cells!(7, 14, 21, 28, 35),
			test_2: create_cells!(14, 21, 28, 35),
			test_3: create_cells!(21, 28, 35),
			test_4: create_cells!(28, 35),
			test_5: create_cells!(35),
			test_6: create_cells!(),
		}
	}

	mod piece_offset {
		use super::super::*;

		macro_rules! test_panic {
			($($name:ident: [$cells:expr, $column:expr],)*) => ($(
				#[test]
				#[should_panic]
				fn $name() {
					let board = AiBoard::new($cells);
					board.piece_offset($column);
				}
			)*);
		}

		test_panic! {
			test_column_offset_overflow: [create_cells!(0, 7, 14, 21, 28, 35), 0],
			test_invalid_offset: [create_cells!(), 7],
		}

		macro_rules! generate_test {
			($($name:ident: [$cells:expr, $offset:expr],)*) => {
			$(
				#[test]
				fn $name() {
					let board = AiBoard::new($cells);
					assert_eq!(board.piece_offset(0), $offset);
				}
			)*
			}
		}

		generate_test! {
			// test_0: omitted because out-of-range
			test_1: [create_cells!(7, 14, 21, 28, 35), 0],
			test_2: [create_cells!(14, 21, 28, 35), 7],
			test_3: [create_cells!(21, 28, 35), 14],
			test_4: [create_cells!(28, 35), 21],
			test_5: [create_cells!(35), 28],
			test_6: [create_cells!(), 35],
		}
	}

	mod piece_add {
		use super::super::*;

		macro_rules! test_panic {
			($($name:ident: [$cells:expr, $column:expr, $offset:expr],)*) => ($(
				#[test]
				#[should_panic]
				fn $name() {
					let mut board = AiBoard::new($cells);
					board.piece_add($column, $offset, Players::Player);
				}
			)*);
		}

		test_panic! {
			test_column_out_of_range: [create_cells!(), 7, 7],
			test_column_full: [create_cells!(0, 7, 14, 21, 28, 35), 0, 0],
			test_offset_out_of_range: [create_cells!(), 0, 42],
			test_already_set: [create_cells!(0), 0, 0],
		}

		#[test]
		#[should_panic]
		fn test_0() {
			let mut board = AiBoard::new(create_cells!(0, 7, 14, 21, 28, 35));
			board.piece_add(0, 0, Players::Player);
		}

		macro_rules! generate_test {
			($($name:ident: [$cells:expr, $offset:expr],)*) => {
			$(
				#[test]
				fn $name() {
					let mut board = AiBoard::new($cells);
					board.piece_add(0, $offset, Players::Player);

					assert_eq!(board.cells[$offset], Players::Player);
				}
			)*
			}
		}

		generate_test! {
			test_1: [create_cells!(7, 14, 21, 28, 35), 0],
			test_2: [create_cells!(14, 21, 28, 35), 7],
			test_3: [create_cells!(21, 28, 35), 14],
			test_4: [create_cells!(28, 35), 21],
			test_5: [create_cells!(35), 28],
			test_6: [create_cells!(), 35],
		}
	}

	mod piece_remove {
		use super::super::*;

		macro_rules! test_panic {
			($($name:ident: [$cells:expr, $column:expr, $offset:expr],)*) => ($(
				#[test]
				#[should_panic]
				fn $name() {
					let mut board = AiBoard::new($cells);
					board.piece_remove($column, $offset);
				}
			)*);
		}

		test_panic! {
			test_column_out_of_range: [create_cells!(), 7, 7],
			test_column_empty: [create_cells!(), 0, 35],
			test_offset_out_of_range: [create_cells!(0), 0, 42],
			test_already_unset: [create_cells!(35), 0, 0],
		}

		macro_rules! generate_test {
			($($name:ident: [$cells:expr, $offset:expr],)*) => ($(
				#[test]
				fn $name() {
					let mut board = AiBoard::new($cells);
					board.piece_remove(0, $offset);

					assert_eq!(board.cells[$offset], Players::Unset);
				}
			)*);
		}

		generate_test! {
			test_0: [create_cells!(0, 7, 14, 21, 28, 35), 0],
			test_1: [create_cells!(7, 14, 21, 28, 35), 7],
			test_2: [create_cells!(14, 21, 28, 35), 14],
			test_3: [create_cells!(21, 28, 35), 21],
			test_4: [create_cells!(28, 35), 28],
			test_5: [create_cells!(35), 35],
		}
	}

	// TODO: min assertions
	// TODO: max assertions
	// TODO: max_top assertions
}
