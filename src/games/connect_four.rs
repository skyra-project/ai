use std::cmp;

use napi::{bindgen_prelude::Uint8Array, Error, Result};

use crate::{isize_to_usize, many_eq, napi_assert, Player, U_INVALID_INDEX};

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

pub type AiCells = [Player; BOARD_CELLS];
pub type AiRemaining = [u8; BOARD_WIDTH];

#[napi]
pub struct ConnectFour {
	cells: AiCells,
	remaining: AiRemaining,
	empty: u8,
}

/// Checks a series of pointer offsets to see if any overlapping groups of 4 are
/// equal.
///
/// This macro assumes `cells` is an expression resolving to an indexable
/// collection of [`Player`]. `cell` therefore must be a `usize`, and all
/// `offset`s are assumed to be an `isize`.
///
/// Note that in debug mode, this makes assertions of `O(1.5n)` time, but it
/// does not in release mode for optimizations. It is also block-safe.
///
/// # Safety
///
/// Undefined behaviour is caused if the pointer offsets added to the origin
/// cell fall outside of the range [0,[`BOARD_CELLS`]), as this causes the
/// resulting pointers to overshoot the cell collection.
macro_rules! check_offsets {
	($cells:expr, $cell:expr, $($offset:expr),+ $(,)?) => {
		{
			if cfg!(debug_assertions) {
				use itertools::{MinMaxResult, Itertools};

				let (min, max) = match [$($offset),+].iter().minmax() {
					MinMaxResult::MinMax(a, b) => (*a, *b),
					MinMaxResult::OneElement(a) => (*a, *a),
					_ => std::hint::unreachable_unchecked(),
				};
				if ($cell as isize + min < 0) || ($cell as isize + max >= BOARD_CELLS as isize) {
					panic!("{} + [{}, {}] offsets fall outside of cell bounds [0, {}]", $cell, min, max, BOARD_CELLS)
				}
				assert_ne!($cells[$cell], Player::Unset);
			}

			let ptr = $cells.as_ptr().add($cell);
			[$($offset),+].map(|x| *ptr.offset(x)).windows(4).any(|x| many_eq!(x[0], x[1], x[2], x[3]))
		}
	};
}

impl ConnectFour {
	fn new(cells: AiCells) -> Self {
		let remaining: [u8; BOARD_WIDTH] = unsafe {
			(0..BOARD_WIDTH)
				.map(|x| (0..BOARD_HEIGHT).take_while(|y| cells[y * BOARD_WIDTH + x] == Player::Unset).count() as u8)
				.collect::<Vec<_>>()
				.try_into()
				.unwrap_unchecked()
		};
		let empty = remaining.iter().sum();

		Self { cells, remaining, empty }
	}

	#[allow(clippy::too_many_arguments)]
	fn status_row(
		&self,
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
			0b0011 => unsafe { check_offsets!(self.cells, cell, 0, r1, r2, r3) },
			0b0111 => unsafe { check_offsets!(self.cells, cell, l1, 0, r1, r2, r3) },
			0b1011 => unsafe { check_offsets!(self.cells, cell, l2, l1, 0, r1, r2, r3) },
			// 0b11xx:
			0b1100 => unsafe { check_offsets!(self.cells, cell, l3, l2, l1, 0) },
			0b1101 => unsafe { check_offsets!(self.cells, cell, l3, l2, l1, 0, r1) },
			0b1110 => unsafe { check_offsets!(self.cells, cell, l3, l2, l1, 0, r1, r2) },
			0b1111 => unsafe { check_offsets!(self.cells, cell, l3, l2, l1, 0, r1, r2, r3) },

			// 0b01xx \ {0b0100, 0b0101} -> {0b0110, [0b0111]}:
			0b0110 => unsafe { check_offsets!(self.cells, cell, l1, 0, r1, r2) },
			// 0b10xx \ {0b1000} -> {0b1001, 0b1010, [0b1011]}:
			0b1001 => unsafe { check_offsets!(self.cells, cell, l2, l1, 0, r1) },
			0b1010 => unsafe { check_offsets!(self.cells, cell, l2, l1, 0, r1, r2) },
			// 0bxx01 \ {0b0001, 0b0101} -> {[0b1001], [0b1101]}:
			// 0bxx10 \ {0b0010} -> {[0b0110], [0b1010], [0b1100]}:

			// Handle all other cases:
			_ => false,
		}
	}

	fn status_vertical(&self, cell: usize) -> bool {
		const B1: isize = BOARD_WIDTH as isize;
		const B2: isize = B1 * 2;
		const B3: isize = B1 * 3;

		AVAILABLE_BOTTOM[cell] == 3 && unsafe { check_offsets!(self.cells, cell, 0, B1, B2, B3) }
	}

	fn status_horizontal(&self, cell: usize) -> bool {
		const L1: isize = -1;
		const L2: isize = L1 * 2;
		const L3: isize = L1 * 3;
		const R1: isize = 1;
		const R2: isize = R1 * 2;
		const R3: isize = R1 * 3;

		self.status_row(AVAILABLE_HORIZONTAL[cell], cell, L1, L2, L3, R1, R2, R3)
	}

	fn status_diagonal_tl(&self, cell: usize) -> bool {
		const TL1: isize = -(BOARD_WIDTH as isize) - 1;
		const TL2: isize = TL1 * 2;
		const TL3: isize = TL1 * 3;
		const BR1: isize = (BOARD_WIDTH as isize) + 1;
		const BR2: isize = BR1 * 2;
		const BR3: isize = BR1 * 3;

		self.status_row(AVAILABLE_DIAGONAL_TL[cell], cell, TL1, TL2, TL3, BR1, BR2, BR3)
	}

	fn status_diagonal_bl(&self, cell: usize) -> bool {
		const BL1: isize = (BOARD_WIDTH as isize) - 1;
		const BL2: isize = BL1 * 2;
		const BL3: isize = BL1 * 3;
		const TR1: isize = -(BOARD_WIDTH as isize) + 1;
		const TR2: isize = TR1 * 2;
		const TR3: isize = TR1 * 3;

		self.status_row(AVAILABLE_DIAGONAL_BL[cell], cell, BL1, BL2, BL3, TR1, TR2, TR3)
	}

	fn status(&self, last_cell_offset: usize) -> bool {
		debug_assert!(last_cell_offset < BOARD_CELLS);
		debug_assert!(self.cells[last_cell_offset] != Player::Unset);

		self.status_vertical(last_cell_offset)
			|| self.status_horizontal(last_cell_offset)
			|| self.status_diagonal_bl(last_cell_offset)
			|| self.status_diagonal_tl(last_cell_offset)
	}

	fn available(&self, column: usize) -> bool {
		debug_assert!(column < BOARD_WIDTH);

		self.remaining[column] > 0
	}

	fn piece_offset(&self, column: usize) -> usize {
		debug_assert!(self.available(column));

		((self.remaining[column] as usize - 1) * BOARD_WIDTH) + column
	}

	fn add(&mut self, column: usize, offset: usize, player: Player) {
		debug_assert!(self.available(column));
		debug_assert!(offset < BOARD_CELLS);
		debug_assert!(self.cells[offset] == Player::Unset);

		self.remaining[column] -= 1;
		self.cells[offset] = player;
	}

	fn remove(&mut self, column: usize, offset: usize) {
		debug_assert!(column < BOARD_WIDTH);
		debug_assert!(self.remaining[column] != BOARD_HEIGHT as u8);
		debug_assert!(offset < BOARD_CELLS);
		debug_assert!(self.cells[offset] != Player::Unset);

		self.remaining[column] += 1;
		self.cells[offset] = Player::Unset;
	}

	/// Minimum is `Player::Human`
	fn min(&mut self, last_cell_offset: usize, remaining: u8, alpha: i8, beta: i8) -> i8 {
		if self.status(last_cell_offset) {
			return OUTCOME_MACHINE_WINS;
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
		let mut min_v = i8::MAX;
		let mut local_beta = beta;

		for c in 0..BOARD_WIDTH {
			if !self.available(c) {
				continue;
			}

			let offset = self.piece_offset(c);

			// On the empty field player Human makes a move and calls Max
			// That's one branch of the game tree:
			self.add(c, offset, Player::Human);

			let m = self.max(offset, remaining - 1, alpha, local_beta);

			// Setting back the field to empty:
			self.remove(c, offset);

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

	/// Maximum is `Player::Machine`
	fn max(&mut self, last_cell_offset: usize, remaining: u8, alpha: i8, beta: i8) -> i8 {
		if self.status(last_cell_offset) {
			return OUTCOME_HUMAN_WINS;
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
			if !self.available(c) {
				continue;
			}

			let offset = self.piece_offset(c);

			// On the empty field player Machine makes a move and calls Min
			// That's one branch of the game tree:
			self.add(c, offset, Player::Machine);

			let m = self.min(offset, remaining - 1, local_alpha, beta);

			// Setting back the field to empty:
			self.remove(c, offset);

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

	fn max_top(&mut self, remaining: u8) -> usize {
		if remaining == 0 {
			return U_INVALID_INDEX;
		}

		const DEFAULT_ALPHA: i8 = i8::MIN;
		const DEFAULT_BETA: i8 = i8::MAX;

		let mut max_v = i8::MIN;
		let mut column = U_INVALID_INDEX;
		for c in 0..BOARD_WIDTH {
			if !self.available(c) {
				continue;
			}

			let offset = self.piece_offset(c);

			// On the empty field player Machine makes a move and calls Min
			// That's one branch of the game tree:
			self.add(c, offset, Player::Machine);

			let points = self.min(offset, remaining, DEFAULT_ALPHA, DEFAULT_BETA);

			// Setting back the field to empty:
			self.remove(c, offset);

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

	fn get_best_move(&mut self, maximum_depth: u8) -> usize {
		// If remaining is 42, then the board is empty.
		//
		// Strategically speaking, the middle position in ConnectFour is always the
		// best, and very often a winner move. The algorithm will always pick this.
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
		// Hardcoding this is useful, on an empty board, there are 4,531,985,219,092
		// possibilities.
		if self.empty == 42 {
			3
		} else {
			// Process the best move for the AI.
			self.max_top(cmp::min(self.empty, maximum_depth))
		}
	}
}

#[napi]
impl ConnectFour {
	#[napi(constructor)]
	pub fn js_new(values: Option<Uint8Array>) -> Result<Self> {
		if let Some(v) = values {
			let input = v.to_vec();
			if input.len() != BOARD_CELLS {
				return Err(Error::from_reason("data must have exactly 42 numbers"));
			}

			let mut cells: AiCells = [Player::Unset; BOARD_CELLS];
			for i in 0..BOARD_CELLS {
				cells[i] = Player::try_from(input[i]).map_err(Error::from_reason)?;
			}

			Ok(ConnectFour::new(cells))
		} else {
			Ok(Self {
				cells: [Player::Unset; BOARD_CELLS],
				remaining: [BOARD_HEIGHT as u8; BOARD_WIDTH],
				empty: BOARD_CELLS as u8,
			})
		}
	}

	#[napi(getter = board)]
	pub fn js_get_board(&self) -> Uint8Array {
		Uint8Array::new(self.cells.map(|v| v as u8).to_vec())
	}

	#[napi(js_name = "available")]
	pub fn js_available(&self, column: i32) -> Result<bool> {
		Ok(self.available(isize_to_usize!(column, BOARD_WIDTH)?))
	}

	#[napi(getter = finished)]
	pub fn js_finished(&self) -> bool {
		self.empty == 0
	}

	#[napi(js_name = "add")]
	pub fn js_add(&mut self, column: i32, player: Player) -> Result<bool> {
		let c = isize_to_usize!(column, BOARD_WIDTH)?;
		napi_assert!(self.remaining[c] > 0);

		let offset = self.piece_offset(c);
		self.add(c, offset, player);
		self.empty -= 1;
		Ok(self.status(offset))
	}

	#[napi(js_name = "getBestMove")]
	pub fn js_get_best_move(&mut self, depth: Option<i32>) -> Result<i32> {
		Ok(self.get_best_move(depth.unwrap_or(5).try_into().unwrap()).try_into().unwrap())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::many_eq;

	#[test]
	fn test_compare_same() {
		assert!(many_eq!(Player::Human, Player::Human, Player::Human, Player::Human));
	}

	macro_rules! compare_different {
		($($name:ident: $value:expr,)*) => {
		$(
			#[test]
			fn $name() {
				let (a, b, c, d) = $value;
				assert!(!many_eq!(a, b, c, d));
			}
		)*
		}
	}

	compare_different! {
		test_compare_one_a_different: (Player::Machine, Player::Human, Player::Human, Player::Human),
		test_compare_one_b_different: (Player::Human, Player::Machine, Player::Human, Player::Human),
		test_compare_one_c_different: (Player::Human, Player::Human, Player::Machine, Player::Human),
		test_compare_one_d_different: (Player::Human, Player::Human, Player::Human, Player::Machine),
	}

	macro_rules! create_cells {
		() => ([Player::Unset; 42]);
		($($index:expr $(, $tail:expr)*)*) => ($({
			let mut tmp = create_cells!($($tail),*);
			tmp[$index] = Player::Human;
			tmp
		})*);
	}

	mod new {
		use super::*;

		#[test]
		fn test_empty() {
			let cells = create_cells!();
			let remaining: AiRemaining = [6; 7];
			let board = ConnectFour::new(cells);

			assert_eq!(board.cells, cells);
			assert_eq!(board.remaining, remaining);
		}

		#[test]
		fn test_row_filled() {
			let cells = create_cells!(35, 36, 37, 38, 39, 40, 41);
			let remaining: AiRemaining = [5; 7];
			let board = ConnectFour::new(cells);

			assert_eq!(board.cells, cells);
			assert_eq!(board.remaining, remaining);
		}

		#[test]
		fn test_column_filled() {
			let cells = create_cells!(0, 7, 14, 21, 28, 35);
			let remaining: AiRemaining = [0, 6, 6, 6, 6, 6, 6];
			let board = ConnectFour::new(cells);

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
					let board = ConnectFour::new($cells);

					unsafe {
						check_offsets!(board.cells, $last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3);
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
			let board = ConnectFour::new(create_cells!(0, 2, 3, 4));

			unsafe {
				assert!(!check_offsets!(board.cells, 0, 0, 1, 2, 3));
			}
		}

		macro_rules! test_true {
			($($name:ident: [$last_column:expr, $offsets:expr],)*) => ($(
				#[test]
				fn $name() {
					let board = ConnectFour::new(create_cells!(0, 1, 2, 3, 4));

					unsafe {
						assert!(check_offsets!(board.cells, $last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3));
					}
				}
			)*);
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
					let board = ConnectFour::new($cells);

					unsafe {
						check_offsets!(board.cells, $last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3, $offsets.4);
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
			let board = ConnectFour::new(create_cells!(0, 2, 3, 4));

			unsafe {
				assert!(!check_offsets!(board.cells, 2, -1, 0, 1, 2, 3));
			}
		}

		macro_rules! test_true {
			($($name:ident: [$last_column:expr, $offsets:expr],)*) => {
			$(
				#[test]
				fn $name() {
					let board = ConnectFour::new(create_cells!(0, 1, 2, 3));

					unsafe {
						assert!(check_offsets!(board.cells, $last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3, $offsets.4));
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
					let board = ConnectFour::new($cells);

					unsafe {
						check_offsets!(board.cells, $last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3, $offsets.4, $offsets.5);
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
			let board = ConnectFour::new(create_cells!(0, 2, 3, 4));

			unsafe {
				assert!(!check_offsets!(board.cells, 2, -2, -1, 0, 1, 2, 3));
			}
		}

		macro_rules! test_true {
			($($name:ident: [$last_column:expr, $offsets:expr],)*) => {
			$(
				#[test]
				fn $name() {
					let board = ConnectFour::new(create_cells!(0, 1, 2, 3, 4));

					unsafe {
						assert!(check_offsets!(board.cells, $last_column, $offsets.0, $offsets.1, $offsets.2, $offsets.3, $offsets.4, $offsets.5));
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
					let board = ConnectFour::new($cells);

					unsafe {
						check_offsets!(board.cells, $cell, -3, -2, -1, 0, 1, 2, 3);
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
			let board = ConnectFour::new(create_cells!(0, 2, 3, 4));

			unsafe {
				assert!(!check_offsets!(board.cells, 3, -3, -2, -1, 0, 1, 2, 3));
			}
		}

		macro_rules! test_true {
			($($name:ident: [$last_column:expr],)*) => {
			$(
				#[test]
				fn $name() {
					let board = ConnectFour::new(create_cells!(0, 1, 2, 3, 4));

					unsafe {
						assert!(check_offsets!(board.cells, $last_column, -3, -2, -1, 0, 1, 2, 3));
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
					let board = ConnectFour::new($cells);
					board.status($last_cell_offset);
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
					let board = ConnectFour::new($cells);
					assert!(!board.status($last_cell_offset));
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
					let board = ConnectFour::new($cells);
					assert!(board.status($last_cell_offset));
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

	mod available {
		use super::super::*;

		#[test]
		#[should_panic]
		fn test_out_of_range_over() {
			let board = ConnectFour::new(create_cells!());
			board.available(7);
		}

		#[test]
		fn test_0() {
			let board = ConnectFour::new(create_cells!(0, 7, 14, 21, 28, 35));
			assert!(!board.available(0));
		}

		macro_rules! test_true {
			($($name:ident: $cells:expr,)*) => {
			$(
				#[test]
				fn $name() {
					let board = ConnectFour::new($cells);
					assert!(board.available(0));
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
					let board = ConnectFour::new($cells);
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
					let board = ConnectFour::new($cells);
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

	mod add {
		use super::super::*;

		macro_rules! test_panic {
			($($name:ident: [$cells:expr, $column:expr, $offset:expr],)*) => ($(
				#[test]
				#[should_panic]
				fn $name() {
					let mut board = ConnectFour::new($cells);
					board.add($column, $offset, Player::Human);
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
			let mut board = ConnectFour::new(create_cells!(0, 7, 14, 21, 28, 35));
			board.add(0, 0, Player::Human);
		}

		macro_rules! generate_test {
			($($name:ident: [$cells:expr, $offset:expr],)*) => {
			$(
				#[test]
				fn $name() {
					let mut board = ConnectFour::new($cells);
					board.add(0, $offset, Player::Human);

					assert_eq!(board.cells[$offset], Player::Human);
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

	mod remove {
		use super::super::*;

		macro_rules! test_panic {
			($($name:ident: [$cells:expr, $column:expr, $offset:expr],)*) => ($(
				#[test]
				#[should_panic]
				fn $name() {
					let mut board = ConnectFour::new($cells);
					board.remove($column, $offset);
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
					let mut board = ConnectFour::new($cells);
					board.remove(0, $offset);

					assert_eq!(board.cells[$offset], Player::Unset);
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
