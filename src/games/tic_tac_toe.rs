use crate::{Players, U_INVALID_INDEX};

struct AiResults {
	pub points: i8,
	pub position: usize,
}

pub const BOARD_WIDTH: usize = 3;
pub const BOARD_HEIGHT: usize = 3;
pub const BOARD_CELLS: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub type AiBoard = [Players; BOARD_CELLS];

#[inline(always)]
fn compare(a: Players, b: Players, c: Players) -> bool {
	a == b && b == c
}

fn status(board: &AiBoard) -> Players {
	// Check rows
	for i in (0..9).step_by(3) {
		let a = board[i];
		if a == Players::Unset {
			continue;
		}

		if compare(a, board[i + 1], board[i + 2]) {
			return a;
		}
	}

	// Check columns
	for i in 0..3 {
		let a = board[i];
		if a == Players::Unset {
			continue;
		}

		if compare(a, board[i + 3], board[i + 6]) {
			return a;
		}
	}

	let middle = board[4];
	if middle == Players::Unset
		// Check descending diagonal
		|| compare(board[0], middle, board[8])
		// Check ascending diagonal
		|| compare(board[6], middle, board[2])
	{
		middle
	} else {
		Players::Unset
	}
}

fn min(board: &mut AiBoard, remaining: u8, alpha: i8, beta: i8) -> AiResults {
	let winner = status(board);
	if winner == Players::Player {
		return AiResults { points: -1, position: U_INVALID_INDEX };
	}

	if winner == Players::Machine {
		return AiResults { points: 1, position: U_INVALID_INDEX };
	}

	if remaining == 0 {
		return AiResults { points: 0, position: U_INVALID_INDEX };
	}

	// Possible values for minv are:
	// -1 - win
	//  0 - a tie
	//  1 - loss
	//
	// We're initially setting it to 2 as worse than the worst case:
	let mut min_v: i8 = 2;
	let mut position: usize = U_INVALID_INDEX;
	let mut local_beta = beta;

	for i in 0..BOARD_CELLS {
		if board[i] == Players::Unset {
			// On the empty field player Player makes a move and calls Max
			// That's one branch of the game tree:
			board[i] = Players::Player;

			let m = max(board, remaining - 1, alpha, local_beta).points;

			// Fixing the minv value if needed:
			if m < min_v {
				min_v = m;
				position = i;
			}

			// Setting back the field to empty:
			board[i] = Players::Unset;

			if min_v <= alpha {
				break;
			}

			if min_v < local_beta {
				local_beta = min_v;
			}
		}
	}

	AiResults { points: min_v, position }
}

/**
 * Maximum is Players::Machine
 */
fn max(board: &mut AiBoard, remaining: u8, alpha: i8, beta: i8) -> AiResults {
	let winner = status(board);
	if winner == Players::Player {
		return AiResults { points: -1, position: U_INVALID_INDEX };
	}

	if winner == Players::Machine {
		return AiResults { points: 1, position: U_INVALID_INDEX };
	}

	if remaining == 0 {
		return AiResults { points: 0, position: U_INVALID_INDEX };
	}

	// Possible values for maxv are:
	// -1 - loss
	//  0 - a tie
	//  1 - win
	//
	// We're initially setting it to -2 as worse than the worst case:
	let mut max_v: i8 = -2;
	let mut position: usize = U_INVALID_INDEX;
	let mut local_alpha = alpha;

	for i in 0..BOARD_CELLS {
		if board[i] == Players::Unset {
			// On the empty field player Machine makes a move and calls Min
			// That's one branch of the game tree:
			board[i] = Players::Machine;

			let m = min(board, remaining - 1, local_alpha, beta).points;

			// Fixing the maxv value if needed:
			if m > max_v {
				max_v = m;
				position = i;
			}

			// Setting back the field to empty:
			board[i] = Players::Unset;

			if max_v >= beta {
				break;
			}

			if max_v > local_alpha {
				local_alpha = max_v;
			}
		}
	}

	AiResults { points: max_v, position }
}

/**
 * Returns the optimal move from the AI, -1 if no move was possible.
 */
pub fn position(board: &mut AiBoard, remaining: u8) -> usize {
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
		max(board, remaining, -2, 2).position
	}
}
