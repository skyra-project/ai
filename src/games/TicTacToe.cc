#include "games/TicTacToe.hpp"

namespace TicTacToe
{
	struct ai_results
	{
		int_fast8_t points;
		int_fast8_t position;
	};

	bool equals(Players a, Players b, Players c) noexcept
	{
		return a == b && b == c;
	}

	Players status(ai_board &board) noexcept
	{
		// 0 1 2
		// 3 4 5
		// 6 7 8

		Players a;

		// Check rows
		for (uint_fast32_t i = 0; i < 9; i += 3)
		{
			a = board[i];
			if (a == Players::Unset)
				continue;

			if (equals(a, board[i + 1], board[i + 2]))
				return a;
		}

		// Check columns
		for (uint_fast32_t i = 0; i < 3; ++i)
		{
			a = board[i];
			if (a == Players::Unset)
				continue;

			if (equals(a, board[i + 3], board[i + 6]))
				return a;
		}

		// Check descending diagonal
		a = board[0];
		if (a != Players::Unset && equals(a, board[4], board[8]))
			return a;

		// Check ascending diagonal
		a = board[6];
		if (a != Players::Unset && equals(a, board[4], board[2]))
			return a;

		return Players::Unset;
	}

	TicTacToe::ai_results min(ai_board &board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept;
	TicTacToe::ai_results max(ai_board &board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept;

	TicTacToe::ai_results max(ai_board &board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept
	{
		const auto winner = status(board);
		if (winner == Players::Player)
			return {-1, -1};

		if (winner == Players::Machine)
			return {1, -1};

		if (remaining == 0)
			return {0, -1};

		int_fast8_t maxv = -2;
		int_fast8_t posi = -1;

		for (int_fast8_t i = 0; i < static_cast<int_fast8_t>(board.size()); ++i)
		{
			if (board[i] == Players::Unset)
			{
				board[i] = Players::Machine;

				const auto m = TicTacToe::min(board, remaining - 1, alpha, beta).points;
				if (m > maxv)
				{
					maxv = m;
					posi = i;
				}

				board[i] = Players::Unset;

				if (maxv >= beta)
				{
					return {maxv, posi};
				}

				if (maxv > alpha)
				{
					alpha = maxv;
				}
			}
		}

		return {maxv, posi};
	}

	TicTacToe::ai_results min(ai_board &board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept
	{
		const auto winner = status(board);
		if (winner == Players::Player)
			return {-1, -1};

		if (winner == Players::Machine)
			return {1, -1};

		if (remaining == 0)
			return {0, -1};

		int_fast8_t minv = 2;
		int_fast8_t posi = -1;

		for (int_fast8_t i = 0; i < static_cast<int_fast8_t>(board.size()); ++i)
		{
			if (board[i] == Players::Unset)
			{
				board[i] = Players::Player;

				const auto m = TicTacToe::max(board, remaining - 1, alpha, beta).points;
				if (m < minv)
				{
					minv = m;
					posi = i;
				}

				board[i] = Players::Unset;

				if (minv <= alpha)
				{
					return {minv, posi};
				}

				if (minv < beta)
				{
					beta = minv;
				}
			}
		}

		return {minv, posi};
	}

	int_fast8_t position(ai_board &board, int_fast8_t remaining) noexcept
	{
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
		// Hardcoding this is useful, on an empty board, there are 2,439,530,234,167 possibilities.
		// On a board with one move in, there are 84,998,978,956 possibilities. That's a lot less.
		if (remaining == 9) {
			return 4;
		}

		// Process the best move for the AI.
		return max(board, remaining, -2, 2).position;
	}
} // namespace TicTacToe
