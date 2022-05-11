use std::cmp;

use napi::{bindgen_prelude::Uint8Array, Error, Result};

use crate::{isize_to_usize, many_eq, napi_assert, Player, U_INVALID_INDEX};

pub const BOARD_WIDTH: usize = 3;
pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_CELLS: usize = BOARD_WIDTH * BOARD_HEIGHT;

const OUTCOME_HUMAN_WINS: i8 = -20;
const OUTCOME_MACHINE_WINS: i8 = 20;
const OUTCOME_DRAW: i8 = 0;

pub type AiCells = [Player; BOARD_CELLS];

#[napi]
pub struct TicTacToe {
	cells: AiCells,
	empty: u8,
}

impl TicTacToe {
	pub fn new(cells: AiCells) -> Self {
		let empty: u8 = cells.iter().filter(|&&c| c == Player::Unset).count().try_into().unwrap();
		Self { cells, empty }
	}

	fn status_horizontal(&self, cell: usize) -> bool {
		match cell {
			0 | 1 | 2 => many_eq!(self.cells[0], self.cells[1], self.cells[2]),
			3 | 4 | 5 => many_eq!(self.cells[3], self.cells[4], self.cells[5]),
			6 | 7 | 8 => many_eq!(self.cells[6], self.cells[7], self.cells[8]),
			_ => unsafe { std::hint::unreachable_unchecked() },
		}
	}

	fn status_vertical(&self, cell: usize) -> bool {
		match cell {
			0 | 3 | 6 => many_eq!(self.cells[0], self.cells[3], self.cells[6]),
			1 | 4 | 7 => many_eq!(self.cells[1], self.cells[4], self.cells[7]),
			2 | 5 | 8 => many_eq!(self.cells[2], self.cells[5], self.cells[8]),
			_ => unsafe { std::hint::unreachable_unchecked() },
		}
	}

	fn status_diagonal_tl(&self, cell: usize) -> bool {
		match cell {
			0 | 4 | 8 => many_eq!(self.cells[0], self.cells[4], self.cells[8]),
			_ => false,
		}
	}

	fn status_diagonal_bl(&self, cell: usize) -> bool {
		match cell {
			2 | 4 | 6 => many_eq!(self.cells[2], self.cells[4], self.cells[6]),
			_ => false,
		}
	}

	fn status(&self, cell: usize) -> bool {
		debug_assert!(cell < BOARD_CELLS);

		self.status_horizontal(cell)
			|| self.status_vertical(cell)
			|| self.status_diagonal_bl(cell)
			|| self.status_diagonal_tl(cell)
	}

	fn available(&self, cell: usize) -> bool {
		debug_assert!(cell < BOARD_CELLS);

		self.cells[cell] == Player::Unset
	}

	fn add(&mut self, cell: usize, player: Player) {
		debug_assert!(cell < BOARD_CELLS);
		debug_assert!(self.cells[cell] == Player::Unset);

		self.cells[cell] = player;
	}

	fn remove(&mut self, cell: usize) {
		debug_assert!(cell < BOARD_CELLS);
		debug_assert!(self.cells[cell] != Player::Unset);

		self.cells[cell] = Player::Unset;
	}

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
		let mut min_v: i8 = i8::MAX;
		let mut local_beta = beta;

		for cell in 0..BOARD_CELLS {
			if self.cells[cell] == Player::Unset {
				// On the empty field player Human makes a move and calls Max
				// That's one branch of the game tree:
				self.add(cell, Player::Human);

				let m = self.max(cell, remaining - 1, alpha, local_beta);

				// Setting back the field to empty:
				self.remove(cell);

				// Fixing the min_v value if needed:
				if m < min_v {
					min_v = m;

					local_beta = cmp::min(local_beta, min_v);
					if alpha >= local_beta {
						break;
					}
				}
			}
		}

		min_v
	}

	/// Maximum is Player::Machine
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
		let mut max_v: i8 = i8::MIN;
		let mut local_alpha = alpha;

		for cell in 0..BOARD_CELLS {
			if self.cells[cell] == Player::Unset {
				// On the empty field player Machine makes a move and calls Min
				// That's one branch of the game tree:
				self.add(cell, Player::Machine);

				let m = self.min(cell, remaining - 1, local_alpha, beta);

				// Setting back the field to empty:
				self.remove(cell);

				// Fixing the max_v value if needed:
				if m > max_v {
					max_v = m;

					local_alpha = cmp::max(local_alpha, max_v);
					if local_alpha >= beta {
						break;
					}
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
		for cell in 0..BOARD_WIDTH {
			if !self.available(cell) {
				continue;
			}

			// On the empty field player Machine makes a move and calls Min
			// That's one branch of the game tree:
			self.add(cell, Player::Machine);

			let points = self.min(cell, remaining, DEFAULT_ALPHA, DEFAULT_BETA);

			// Setting back the field to empty:
			self.remove(cell);

			if points > max_v {
				max_v = points;
				column = cell;

				// Break the loop earlier if we have found a winning move:
				if points >= OUTCOME_MACHINE_WINS {
					break;
				}
			}
		}

		column
	}

	/// Returns the optimal move from the AI, -1 if no move was possible.
	pub fn get_best_move(&mut self, maximum_depth: u8) -> usize {
		// If remaining is 9, then the board is empty.
		//
		// Strategically speaking, the middle position in TicTacToe is always the best,
		// and very often a winner move. The algorithm will always pick this.
		//
		// We have this board:
		// 0 | 1 | 2
		// --+---+--
		// 3 | 4 | 5
		// --+---+--
		// 6 | 7 | 8
		//
		// The center is 4, therefore, we return this number.
		//
		// Hardcoding this is useful, on an empty board, there are 362,880
		// possibilities. On a board with one move in, there are 40,320 possibilities.
		// That's a lot less.
		if self.empty == 9 {
			4
		} else {
			self.max_top(cmp::min(self.empty, maximum_depth))
		}
	}
}

#[napi]
impl TicTacToe {
	#[napi(constructor)]
	pub fn js_new(values: Option<Uint8Array>) -> Result<Self> {
		if let Some(v) = values {
			let input = v.to_vec();
			if input.len() != BOARD_CELLS {
				return Err(Error::from_reason("data must have exactly 9 numbers"));
			}

			let mut cells: AiCells = [Player::Unset; BOARD_CELLS];
			for i in 0..BOARD_CELLS {
				cells[i] = Player::try_from(input[i]).map_err(Error::from_reason)?;
			}

			Ok(TicTacToe::new(cells))
		} else {
			Ok(Self { cells: [Player::Unset; BOARD_CELLS], empty: BOARD_CELLS as u8 })
		}
	}

	#[napi(getter = board)]
	pub fn js_get_board(&self) -> Uint8Array {
		Uint8Array::new(self.cells.map(|v| v as u8).to_vec())
	}

	#[napi(js_name = "available")]
	pub fn js_available(&self, cell: i32) -> Result<bool> {
		Ok(self.available(isize_to_usize!(cell, BOARD_CELLS)?))
	}

	#[napi(getter = finished)]
	pub fn js_finished(&self) -> bool {
		self.empty == 0
	}

	#[napi(js_name = "add")]
	pub fn js_add(&mut self, cell: i32, player: Player) -> Result<bool> {
		let c = isize_to_usize!(cell, BOARD_CELLS)?;
		napi_assert!(self.cells[c] != Player::Unset);

		self.add(c, player);
		self.empty -= 1;
		Ok(self.status(c))
	}

	#[napi(js_name = "getBestMove")]
	pub fn js_get_best_move(&mut self, depth: Option<i32>) -> Result<i32> {
		Ok(self.get_best_move(depth.unwrap_or(5).try_into().unwrap()).try_into().unwrap())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! create_cells {
		() => ([Player::Unset; 9]);
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
			let board = TicTacToe::new(cells);

			assert_eq!(board.cells, cells);
			assert_eq!(board.empty, 9);
		}

		#[test]
		fn test_partially_empty() {
			let cells = create_cells!(0, 2, 4);
			let board = TicTacToe::new(cells);

			assert_eq!(board.cells, cells);
			assert_eq!(board.empty, 6);
		}
	}

	mod status_horizontal {
		use super::*;

		macro_rules! test_false {
			($($name:ident: [$cells:expr, $last_cell_offset:expr],)*) => ($(
				#[test]
				fn $name() {
					let board = TicTacToe::new($cells);
					assert!(!board.status_horizontal($last_cell_offset));
				}
			)*);
		}

		test_false! {
			test_fail_0_0_empty: [create_cells!(0), 0],
			test_fail_0_0_not_full: [create_cells!(0, 1), 0],
			test_fail_0_0_rest_full: [create_cells!(0, 3, 4, 5, 6, 7, 8), 0],
		}

		macro_rules! test_true {
			($($name:ident: [$cells:expr, $last_cell_offset:expr],)*) => ($(
				#[test]
				fn $name() {
					let board = TicTacToe::new($cells);
					assert!(board.status_horizontal($last_cell_offset));
				}
			)*);
		}

		test_true! {
			test_pass_0_1_full: [create_cells!(0, 1, 2), 0],
			test_pass_1_1_full: [create_cells!(0, 1, 2), 1],
			test_pass_2_1_full: [create_cells!(0, 1, 2), 2],
			test_pass_0_2_full: [create_cells!(3, 4, 5), 3],
			test_pass_1_2_full: [create_cells!(3, 4, 5), 4],
			test_pass_2_2_full: [create_cells!(3, 4, 5), 5],
			test_pass_0_3_full: [create_cells!(6, 7, 8), 6],
			test_pass_1_3_full: [create_cells!(6, 7, 8), 7],
			test_pass_2_3_full: [create_cells!(6, 7, 8), 8],
		}
	}

	mod status_vertical {
		use super::*;

		macro_rules! test_false {
			($($name:ident: [$cells:expr, $last_cell_offset:expr],)*) => ($(
				#[test]
				fn $name() {
					let board = TicTacToe::new($cells);
					assert!(!board.status_vertical($last_cell_offset));
				}
			)*);
		}

		test_false! {
			test_fail_0_0_empty: [create_cells!(0), 0],
			test_fail_0_0_not_full: [create_cells!(0, 1), 0],
			test_fail_0_0_rest_full: [create_cells!(0, 1, 2, 5, 7, 8), 0],
		}

		macro_rules! test_true {
			($($name:ident: [$cells:expr, $last_cell_offset:expr],)*) => ($(
				#[test]
				fn $name() {
					let board = TicTacToe::new($cells);
					assert!(board.status_vertical($last_cell_offset));
				}
			)*);
		}

		test_true! {
			test_pass_0_1_full: [create_cells!(0, 3, 6), 0],
			test_pass_1_1_full: [create_cells!(0, 3, 6), 3],
			test_pass_2_1_full: [create_cells!(0, 3, 6), 6],
			test_pass_0_2_full: [create_cells!(1, 4, 7), 1],
			test_pass_1_2_full: [create_cells!(1, 4, 7), 4],
			test_pass_2_2_full: [create_cells!(1, 4, 7), 7],
			test_pass_0_3_full: [create_cells!(2, 5, 8), 2],
			test_pass_1_3_full: [create_cells!(2, 5, 8), 5],
			test_pass_2_3_full: [create_cells!(2, 5, 8), 8],
		}
	}

	mod status_diagonal_tl {}

	mod status_diagonal_bl {}

	mod status {}

	mod available {}

	mod add {}

	mod remove {}

	mod min {}

	mod max {}

	mod max_top {}

	mod get_best_move {}
}
