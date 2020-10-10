#include "games/ConnectFour.hpp"
#include <algorithm>

namespace ConnectFour
{
	struct ai_results
	{
		int_fast8_t points;
		int_fast8_t position;
	};

	struct cell_data
	{
		int_fast8_t column;
		int_fast8_t row;
	};

	bool getColumnFilled(ConnectFour::ai_board &board, uint_fast32_t column) noexcept
	{
		return board[column] != Players::Unset;
	}

	bool getBoardFilled(ConnectFour::ai_board &board) noexcept
	{
		for (uint_fast32_t column = 0; column < board_width; ++column)
		{
			if (!getColumnFilled(board, column))
			{
				return false;
			}
		}

		return true;
	}

	bool equals(Players a, Players b, Players c, Players d) noexcept
	{
		return a == b && b == c && c == d;
	}

	bool isInRange(uint_fast32_t value, uint_fast32_t min, uint_fast32_t max) noexcept
	{
		return value >= min && value < max;
	}

	int_fast8_t getFreeRowAtColumn(ConnectFour::ai_board &board, uint_fast32_t column) noexcept
	{
		// Declare the position of the board at which the value would be placed.
		int_fast8_t position = -1;

		// Start at the top of the board, using board and then adding the width of the board to the position.
		for (uint_fast32_t row = column; row < board.size(); row += board_width)
		{
			if (board[row] == Players::Unset)
			{
				position = row;
			}
			else
			{
				break;
			}
		}

		return position;
	}

	bool isWinningHorizontal(ConnectFour::ai_board &board, Players player, uint_fast8_t column, uint_fast8_t row) noexcept
	{
		// When checking horizontal rows, we need to know what is the most left and most right column:
		static constexpr int_fast8_t leftMostColumn = 0;
		static constexpr int_fast8_t rightMostColumn = static_cast<int_fast8_t>(board_width - 1);

		// To optimize the steps, we do not need to know the cells that are over 3 cells to the left, nor the ones 3 cells to the right.
		// We optimize this by obtaining the minimum column as column - 3 and the maximum column as column + 3, limited by the previously calculated range.
		const uint_fast32_t minimumColumn = static_cast<uint_fast32_t>(std::max(static_cast<int_fast8_t>(column - 3), leftMostColumn));
		const uint_fast32_t maximumColumn = static_cast<uint_fast32_t>(std::min(static_cast<int_fast8_t>(column + 3), rightMostColumn));

		// Once we have the columns, we calculate the cells at which we will start and end.
		// Since the data is aligned from left to right and top to bottom, we can easily iterate over all 7 elements in order.
		const uint_fast32_t minimumCell = static_cast<uint_fast32_t>(minimumColumn * row);
		const uint_fast32_t maximumCell = static_cast<uint_fast32_t>(maximumColumn * row);

		// We will count how many cells in a row have the same value as player.
		for (uint_fast32_t i = minimumCell, count = 0; i <= maximumCell; ++i)
		{
			// If the cell's value is the same as player, we add one to the count.
			if (board[i] == player)
			{
				// If there are 4 cells with the same value, we return early with true.
				if (++count == 4)
					return true;
			}
			// Else we will reset the counter to 0.
			else
			{
				count = 0;
			}
		}

		// We did not find 4 cells with the same value in a row, returns false.
		return false;
	}

	bool isWinningVertical(ConnectFour::ai_board &board, Players player, uint_fast8_t column, uint_fast8_t row) noexcept
	{
		// Connect-Four has a 7x6 board, but for a vertical to be winning, it needs at least 4 cells.
		// The lowest row (0) is at the top, and the highest row (5) is at the bottom.
		// Therefore, it needs the rows 5-4-3-2 to be filled.
		//
		// requiredFreeRows equals to 6 - 4 (2) and is computed only once, as it's static. This marks the required
		// row for which this can be checked. If row is lower than 2, then we have a {5-4-3}, {5-4}, or a {5} sequence.
		// None of which are enough for a winning vertical row to exist.
		static constexpr uint_fast8_t requiredFreeRows = static_cast<uint_fast8_t>(board_height - 4);
		if (row > requiredFreeRows)
		{
			return false;
		}

		// We calculate the cell below the one we checked, then we check whether the three values below equals player.
		const uint_fast32_t minimumCell = static_cast<uint_fast32_t>(column * (row + 1));

		// We calculate at compile-time the offset needed for the cells below. We do adds instead of getting the cell
		// from a multiplication to optimize the CPU time.
		static constexpr uint_fast32_t offset1 = board_width;
		static constexpr uint_fast32_t offset2 = board_width + board_width;
		return equals(player, board[minimumCell], board[minimumCell + offset1], board[minimumCell + offset2]);
	}

	bool isWinningDiagonalUpwards(ConnectFour::ai_board &board, Players player, uint_fast8_t column, uint_fast8_t row) noexcept
	{
		// When checking the following diagonal:
		//
		//  00  01  02  03  04 [05] 06
		//  07  08  09  10 [11] 12  13
		//  14  15  16 [17] 18  19  20
		//  21  22 [23] 24  25  26  27
		//  28 [29] 30  31  32  33  34
		// [35] 36 37  38  39  40  41
		//
		// We can see that 11 - 5, 17 - 11, 23 - 17, 29 - 23, and 35 - 29, return the same value: 6, which in turn is
		// the same as board_width - 1:
		static constexpr int_fast32_t offset = board_width - 1;

		// We will also calculate the offset for the edges, so in the case for the number 17, this will
		// add offset * 3, which equals to 17 + (6 * 3) = 17 + 18 = 35, which is right given the figure above:
		static constexpr int_fast32_t edgeValueOffset = offset * 3;

		const int_fast32_t startingCell = static_cast<int_fast32_t>(column * row);
		const int_fast32_t minimumCell = static_cast<int_fast32_t>(startingCell + edgeValueOffset);
		const int_fast32_t maximumCell = static_cast<int_fast32_t>(startingCell - edgeValueOffset);

		// We will count how many cells in a row have the same value as player.
		for (int_fast32_t i = minimumCell, count = 0; i <= maximumCell; i -= offset)
		{
			// If the value is not within range, skip:
			if (!isInRange(i, board_min, board_max))
				continue;

			// If the cell's value is the same as player, we add one to the count.
			if (board[i] == player)
			{
				// If there are 4 cells with the same value, we return early with true.
				if (++count == 4)
					return true;
			}
			// Else we will reset the counter to 0.
			else
			{
				count = 0;
			}
		}

		// We did not find 4 cells with the same value in a row, returns false.
		return false;
	}

	bool isWinningDiagonalDownwards(ConnectFour::ai_board &board, Players player, uint_fast8_t column, uint_fast8_t row) noexcept
	{
		// When checking the following diagonal:
		//
		// [00] 01  02  03  04  05  06
		//  07 [08] 09  10  11  12  13
		//  14  15 [16] 17  18  19  20
		//  21  22  23 [24] 25  26  27
		//  28  29  30  31 [32] 33  34
		//  35  36  37  38  39 [40] 41
		//
		// We can see that 8 - 0, 16 - 8, 24 - 16, 32 - 24, and 40 - 32, return the same value: 8, which in turn is
		// the same as board_width + 1:
		static constexpr int_fast32_t offset = board_width;

		// We will also calculate the offset for the edges, so in the case for the number 16, this will
		// add offset * 3, which equals to 16 + (8 * 3) = 16 + 24 = 40, which is right given the figure above:
		static constexpr int_fast32_t edgeValueOffset = offset * 3;

		const int_fast32_t startingCell = static_cast<int_fast32_t>(column * row);
		const int_fast32_t minimumCell = static_cast<int_fast32_t>(startingCell + edgeValueOffset);
		const int_fast32_t maximumCell = static_cast<int_fast32_t>(startingCell - edgeValueOffset);

		// We will count how many cells in a row have the same value as player.
		for (int_fast32_t i = minimumCell, count = 0; i <= maximumCell; i -= offset)
		{
			// If the value is not within range, skip:
			if (!isInRange(i, board_min, board_max))
				continue;

			// If the cell's value is the same as player, we add one to the count.
			if (board[i] == player)
			{
				// If there are 4 cells with the same value, we return early with true.
				if (++count == 4)
					return true;
			}
			// Else we will reset the counter to 0.
			else
			{
				count = 0;
			}
		}

		// We did not find 4 cells with the same value in a row, returns false.
		return false;
	}

	bool isWinningMove(ConnectFour::ai_board &board, Players player, uint_fast8_t column, uint_fast8_t row) noexcept
	{
		return isWinningVertical(board, player, column, row)			 // Check vertical:
			   || isWinningHorizontal(board, player, column, row)		 // Check horizontal:
			   || isWinningDiagonalDownwards(board, player, column, row) // Check diagonals:
			   || isWinningDiagonalUpwards(board, player, column, row);
	}

	ConnectFour::ai_results min(ai_board &board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept;
	ConnectFour::ai_results max(ai_board &board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept;

	// Maximum is Players::Machine
	ConnectFour::ai_results max(ai_board &board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept
	{
		// TODO(kyranet): Use isWinningMove here:

		// const auto winner = status(board);
		// if (winner == Players::Player)
		// 	return {-1, -1};

		// if (winner == Players::Machine)
		// 	return {1, -1};

		// if (remaining == 0)
		// 	return {0, -1};

		// Possible values for maxv are:
		// -1 - loss
		//  0 - a tie
		//  1 - win
		//
		// We're initially setting it to -2 as worse than the worst case:
		int_fast8_t maxv = -2;
		int_fast8_t posi = -1;

		for (int_fast8_t i = 0; i < static_cast<int_fast8_t>(board.size()); ++i)
		{
			if (board[i] == Players::Unset)
			{
				// On the empty field player Machine makes a move and calls Min
				// That's one branch of the game tree:
				board[i] = Players::Machine;

				const auto m = ConnectFour::min(board, remaining - 1, alpha, beta).points;

				// Fixing the maxv value if needed:
				if (m > maxv)
				{
					maxv = m;
					posi = i;
				}

				// Setting back the field to empty:
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

	// Minimum is Players::Player
	ConnectFour::ai_results min(ai_board &board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept
	{
		// TODO(kyranet): Use isWinningMove here:

		// const auto winner = status(board);
		// if (winner == Players::Player)
		// 	return {-1, -1};

		// if (winner == Players::Machine)
		// 	return {1, -1};

		// if (remaining == 0)
		// 	return {0, -1};

		// Possible values for minv are:
		// -1 - win
		//  0 - a tie
		//  1 - loss
		//
		// We're initially setting it to 2 as worse than the worst case:
		int_fast8_t minv = 2;
		int_fast8_t posi = -1;

		for (int_fast8_t i = 0; i < static_cast<int_fast8_t>(board.size()); ++i)
		{
			if (board[i] == Players::Unset)
			{
				// On the empty field player Player makes a move and calls Max
				// That's one branch of the game tree:
				board[i] = Players::Player;

				const auto m = ConnectFour::max(board, remaining - 1, alpha, beta).points;

				// Fixing the minv value if needed:
				if (m < minv)
				{
					minv = m;
					posi = i;
				}

				// Setting back the field to empty:
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

	// Returns the optimal move from the AI, -1 if no move was possible.
	int_fast8_t position(ai_board &board, int_fast8_t remaining) noexcept
	{
		// If remaining is 9, then the board is empty.
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
		// Hardcoding this is useful, on an empty board, there are 2,439,530,234,167 possibilities.
		// On a board with one move in, there are 84,998,978,956 possibilities. That's a lot less.
		if (remaining == 42)
		{
			return 4;
		}

		// Process the best move for the AI.
		return max(board, remaining, -2, 2).position;
	}
} // namespace ConnectFour
