use crate::{Players, U_INVALID_INDEX};
use std::cmp;

pub const BOARD_WIDTH: usize = 3;
pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_CELLS: usize = BOARD_WIDTH * BOARD_HEIGHT;

const OUTCOME_HUMAN_WINS: i8 = -20;
const OUTCOME_MACHINE_WINS: i8 = 20;
const OUTCOME_DRAW: i8 = 0;

pub type AiCells = [Players; BOARD_CELLS];

#[inline(always)]
fn compare(a: Players, b: Players, c: Players) -> bool {
	a == b && b == c
}

pub struct AiBoard {
	pub cells: AiCells,
}

impl AiBoard {
	pub fn new(cells: AiCells) -> Self {
		Self { cells }
	}

	fn status(&self) -> Players {
		// Check rows
		for i in (0..9).step_by(3) {
			let a = self.cells[i];
			if a == Players::Unset {
				continue;
			}

			if compare(a, self.cells[i + 1], self.cells[i + 2]) {
				return a;
			}
		}

		// Check columns
		for i in 0..3 {
			let a = self.cells[i];
			if a == Players::Unset {
				continue;
			}

			if compare(a, self.cells[i + 3], self.cells[i + 6]) {
				return a;
			}
		}

		let middle = self.cells[4];
		if middle == Players::Unset
			// Check descending diagonal
			|| compare(self.cells[0], middle, self.cells[8])
			// Check ascending diagonal
			|| compare(self.cells[6], middle, self.cells[2])
		{
			middle
		} else {
			Players::Unset
		}
	}

	fn available(&self, cell: usize) -> bool {
		debug_assert!(cell < BOARD_CELLS);

		self.cells[cell] == Players::Unset
	}

	fn add(&mut self, cell: usize, player: Players) {
		debug_assert!(cell < BOARD_CELLS);
		debug_assert!(self.cells[cell] == Players::Unset);

		self.cells[cell] = player;
	}

	fn remove(&mut self, cell: usize) {
		debug_assert!(cell < BOARD_CELLS);
		debug_assert!(self.cells[cell] != Players::Unset);

		self.cells[cell] = Players::Unset;
	}

	fn min(&mut self, remaining: u8, alpha: i8, beta: i8) -> i8 {
		if self.status() == Players::Machine {
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
			if self.cells[cell] == Players::Unset {
				// On the empty field player Player makes a move and calls Max
				// That's one branch of the game tree:
				self.add(cell, Players::Player);

				let m = self.max(remaining - 1, alpha, local_beta);

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

	/// Maximum is Players::Machine
	fn max(&mut self, remaining: u8, alpha: i8, beta: i8) -> i8 {
		if self.status() == Players::Player {
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
			if self.cells[cell] == Players::Unset {
				// On the empty field player Machine makes a move and calls Min
				// That's one branch of the game tree:
				self.add(cell, Players::Machine);

				let m = self.min(remaining - 1, local_alpha, beta);

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
		for c in 0..BOARD_WIDTH {
			if !self.available(c) {
				continue;
			}

			// On the empty field player Machine makes a move and calls Min
			// That's one branch of the game tree:
			self.add(c, Players::Machine);

			let points = self.min(remaining, DEFAULT_ALPHA, DEFAULT_BETA);

			// Setting back the field to empty:
			self.remove(c);

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

	/// Returns the optimal move from the AI, -1 if no move was possible.
	pub fn get_best_move(&mut self, remaining: u8) -> usize {
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
		// Hardcoding this is useful, on an empty board, there are 362,880 possibilities.
		// On a board with one move in, there are 40,320 possibilities. That's a lot less.
		if remaining == 9 {
			4
		} else {
			self.max_top(remaining)
		}
	}
}
