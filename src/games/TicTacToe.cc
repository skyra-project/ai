#include "games/TicTacToe.hpp"

namespace tic_tac_toe {
  struct ai_results {
    int_fast8_t points;
    int_fast8_t position;
  };

  bool equals(Players a, Players b, Players c) noexcept { return a == b && b == c; }

  Players status(ai_board& board) noexcept {
    // 0 1 2
    // 3 4 5
    // 6 7 8

    Players a;

    // Check rows
    for (uint_fast32_t i = 0; i < 9; i += 3) {
      a = board[i];
      if (a == Players::Unset)
        continue;

      if (equals(a, board[i + 1], board[i + 2]))
        return a;
    }

    // Check columns
    for (uint_fast32_t i = 0; i < 3; ++i) {
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

  ai_results min(ai_board& board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept;
  ai_results max(ai_board& board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept;

  // Maximum is Players::Machine
  ai_results max(ai_board& board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept {
    const auto winner = status(board);
    if (winner == Players::Player)
      return {-1, -1};

    if (winner == Players::Machine)
      return {1, -1};

    if (remaining == 0)
      return {0, -1};

    // Possible values for maxv are:
    // -1 - loss
    //  0 - a tie
    //  1 - win
    //
    // We're initially setting it to -2 as worse than the worst case:
    int_fast8_t maxv = -2;
    int_fast8_t posi = -1;

    for (int_fast8_t i = 0; i < static_cast<int_fast8_t>(board.size()); ++i) {
      if (board[i] == Players::Unset) {
        // On the empty field player Machine makes a move and calls Min
        // That's one branch of the game tree:
        board[i] = Players::Machine;

        const auto m = tic_tac_toe::min(board, remaining - 1, alpha, beta).points;

        // Fixing the maxv value if needed:
        if (m > maxv) {
          maxv = m;
          posi = i;
        }

        // Setting back the field to empty:
        board[i] = Players::Unset;

        if (maxv >= beta) {
          return {maxv, posi};
        }

        if (maxv > alpha) {
          alpha = maxv;
        }
      }
    }

    return {maxv, posi};
  }

  // Minimum is Players::Player
  ai_results min(ai_board& board, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept {
    const auto winner = status(board);
    if (winner == Players::Player)
      return {-1, -1};

    if (winner == Players::Machine)
      return {1, -1};

    if (remaining == 0)
      return {0, -1};

    // Possible values for minv are:
    // -1 - win
    //  0 - a tie
    //  1 - loss
    //
    // We're initially setting it to 2 as worse than the worst case:
    int_fast8_t minv = 2;
    int_fast8_t posi = -1;

    for (int_fast8_t i = 0; i < static_cast<int_fast8_t>(board.size()); ++i) {
      if (board[i] == Players::Unset) {
        // On the empty field player Player makes a move and calls Max
        // That's one branch of the game tree:
        board[i] = Players::Player;

        const auto m = tic_tac_toe::max(board, remaining - 1, alpha, beta).points;

        // Fixing the minv value if needed:
        if (m < minv) {
          minv = m;
          posi = i;
        }

        // Setting back the field to empty:
        board[i] = Players::Unset;

        if (minv <= alpha) {
          return {minv, posi};
        }

        if (minv < beta) {
          beta = minv;
        }
      }
    }

    return {minv, posi};
  }

  // Returns the optimal move from the AI, -1 if no move was possible.
  int_fast8_t position(ai_board& board, int_fast8_t remaining) noexcept {
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
    if (remaining == 9) {
      return 4;
    }

    // Process the best move for the AI.
    return max(board, remaining, -2, 2).position;
  }
} // namespace tic_tac_toe
