#include "games/ConnectFour.hpp"
#include <bitset>
#include <iostream>
#include <limits>

namespace connect_four {
  std::array<uint_fast8_t, board_cells> available_bottom{     //
      0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, //
      0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, //
      0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, 0b0011, //
      0b0010, 0b0010, 0b0010, 0b0010, 0b0010, 0b0010, 0b0010, //
      0b0001, 0b0001, 0b0001, 0b0001, 0b0001, 0b0001, 0b0001, //
      0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000};
  std::array<uint_fast8_t, board_cells> available_horizontal{ //
      0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100, //
      0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100, //
      0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100, //
      0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100, //
      0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100, //
      0b0011, 0b0111, 0b1011, 0b1111, 0b1110, 0b1101, 0b1100};
  std::array<uint_fast8_t, board_cells> available_diagonal_tl{//
      0b0011, 0b0011, 0b0011, 0b0011, 0b0010, 0b0001, 0b0000, //
      0b0011, 0b0111, 0b0111, 0b0111, 0b0110, 0b0101, 0b0100, //
      0b0011, 0b0111, 0b1011, 0b1011, 0b1010, 0b1001, 0b1000, //
      0b0010, 0b0110, 0b1010, 0b1110, 0b1110, 0b1101, 0b1100, //
      0b0001, 0b0101, 0b1001, 0b1101, 0b1101, 0b1101, 0b1100, //
      0b0000, 0b0100, 0b1000, 0b1100, 0b1100, 0b1100, 0b1100};
  std::array<uint_fast8_t, board_cells> available_diagonal_bl{//
      0b0000, 0b0100, 0b1000, 0b1100, 0b1100, 0b1100, 0b1100, //
      0b0001, 0b0101, 0b1001, 0b1101, 0b1101, 0b1101, 0b1100, //
      0b0010, 0b0110, 0b1010, 0b1110, 0b1110, 0b1101, 0b1100, //
      0b0011, 0b0111, 0b1011, 0b1011, 0b1010, 0b1001, 0b1000, //
      0b0011, 0b0111, 0b0111, 0b0111, 0b0110, 0b0101, 0b0100, //
      0b0011, 0b0011, 0b0011, 0b0011, 0b0010, 0b0001, 0b0000};

  constexpr size_t undefined_last_move{std::numeric_limits<size_t>::max()};
  constexpr int_fast8_t no_column{-1};

  struct ai_results {
    int_fast8_t points;
    int_fast8_t position;
  };

  bool compare(Players a, Players b, Players c, Players d) noexcept { return a == b && b == c && c == d; }

  bool check(const ai_board& board, size_t cell, int_fast8_t a, int_fast8_t b, int_fast8_t c, int_fast8_t d) noexcept {
    const auto ca = board.cells[cell + a];
    const auto cb = board.cells[cell + b];
    const auto cc = board.cells[cell + c];
    const auto cd = board.cells[cell + d];
    return compare(ca, cb, cc, cd);
  }

  bool check(const ai_board& board, size_t cell, int_fast8_t a, int_fast8_t b, int_fast8_t c, int_fast8_t d,
      int_fast8_t e) noexcept {
    const auto ca = board.cells[cell + a];
    const auto cb = board.cells[cell + b];
    const auto cc = board.cells[cell + c];
    const auto cd = board.cells[cell + d];
    const auto ce = board.cells[cell + e];
    return compare(ca, cb, cc, cd) || compare(cb, cc, cd, ce);
  }

  bool check(const ai_board& board, size_t cell, int_fast8_t a, int_fast8_t b, int_fast8_t c, int_fast8_t d,
      int_fast8_t e, int_fast8_t f) noexcept {
    const auto ca = board.cells[cell + a];
    const auto cb = board.cells[cell + b];
    const auto cc = board.cells[cell + c];
    const auto cd = board.cells[cell + d];
    const auto ce = board.cells[cell + e];
    const auto cf = board.cells[cell + f];
    return compare(ca, cb, cc, cd) || compare(cb, cc, cd, ce) || compare(cc, cd, ce, cf);
  }

  bool check(const ai_board& board, size_t cell, int_fast8_t a, int_fast8_t b, int_fast8_t c, int_fast8_t d,
      int_fast8_t e, int_fast8_t f, int_fast8_t g) noexcept {
    const auto ca = board.cells[cell + a];
    const auto cb = board.cells[cell + b];
    const auto cc = board.cells[cell + c];
    const auto cd = board.cells[cell + d];
    const auto ce = board.cells[cell + e];
    const auto cf = board.cells[cell + f];
    const auto cg = board.cells[cell + g];
    return compare(ca, cb, cc, cd) || compare(cb, cc, cd, ce) || compare(cc, cd, ce, cf) || compare(cd, ce, cf, cg);
  }

  bool status_row(const ai_board& board, uint_fast8_t mask, size_t cell, int_fast8_t l1, int_fast8_t l2, int_fast8_t l3,
      int_fast8_t r1, int_fast8_t r2, int_fast8_t r3) noexcept {
    switch (mask) {
      case 0b0000:
      case 0b0001:
      case 0b0010:
      case 0b0100:
      case 0b0101:
      case 0b1000:
        return false;
      case 0b0011:
        return check(board, cell, 0, r1, r2, r3);
      case 0b0111:
        return check(board, cell, l1, 0, r1, r2, r3);
      case 0b1011:
        return check(board, cell, l2, l1, 0, r1, r2, r3);
      case 0b1111:
        return check(board, cell, l3, l2, l1, 0, r1, r2, r3);
      case 0b1100:
        return check(board, cell, l3, l2, l1, 0);
      case 0b1101:
        return check(board, cell, l3, l2, l1, 0, r1);
      case 0b1110:
        return check(board, cell, l3, l2, l1, 0, r1, r2);
      case 0b1001:
        return check(board, cell, l2, l1, 0, r1);
      case 0b1010:
        return check(board, cell, l2, l1, 0, r1, r2);
      case 0b0110:
        return check(board, cell, l1, 0, r1, r2);
      default:
        return false;
    }
  }

  bool status(ai_board& board, size_t cell) noexcept {
    if (cell == undefined_last_move)
      return false;

    // Vertical
    constexpr int_fast8_t b1 = board_width;
    constexpr int_fast8_t b2 = b1 * 2;
    constexpr int_fast8_t b3 = b1 * 3;

    if (available_bottom[cell] == 3 && check(board, cell, 0, b1, b2, b3))
      return true;

    // Horizontal
    constexpr int_fast8_t l1 = -1;
    constexpr int_fast8_t l2 = l1 * 2;
    constexpr int_fast8_t l3 = l1 * 3;
    constexpr int_fast8_t r1 = 1;
    constexpr int_fast8_t r2 = r1 * 2;
    constexpr int_fast8_t r3 = r1 * 3;

    // Top Left to Bottom Right
    constexpr int_fast8_t tl1 = -board_width - 1;
    constexpr int_fast8_t tl2 = tl1 * 2;
    constexpr int_fast8_t tl3 = tl1 * 3;
    constexpr int_fast8_t br1 = board_width + 1;
    constexpr int_fast8_t br2 = br1 * 2;
    constexpr int_fast8_t br3 = br1 * 3;

    // Bottom Left to Top Right
    constexpr int_fast8_t bl1 = board_width - 1;
    constexpr int_fast8_t bl2 = bl1 * 2;
    constexpr int_fast8_t bl3 = bl1 * 3;
    constexpr int_fast8_t tr1 = -board_width + 1;
    constexpr int_fast8_t tr2 = tr1 * 2;
    constexpr int_fast8_t tr3 = tr1 * 3;

    return status_row(board, available_horizontal[cell], cell, l1, l2, l3, r1, r2, r3)           //
           || status_row(board, available_diagonal_tl[cell], cell, tl1, tl2, tl3, br1, br2, br3) //
           || status_row(board, available_diagonal_bl[cell], cell, bl1, bl2, bl3, tr1, tr2, tr3);
  }

  bool column_available(const ai_board& board, int_fast8_t position) noexcept {
    return board.remaining[position] != static_cast<uint_fast8_t>(0);
  }

  size_t piece_offset(ai_board& board, int_fast8_t column) noexcept {
    return static_cast<size_t>(((board.remaining[column] - 1) * board_width) + column);
  }

  void piece_add(ai_board& board, int_fast8_t column, size_t offset, Players player) noexcept {
    --board.remaining[column];
    board.cells[offset] = player;
  }

  void piece_remove(ai_board& board, int_fast8_t column, size_t offset) noexcept {
    ++board.remaining[column];
    board.cells[offset] = Players::Unset;
  }

  ai_results min(
      ai_board& board, size_t last_move, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept;
  ai_results max(
      ai_board& board, size_t last_move, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept;

  // Maximum is Players::Machine
  ai_results max(
      ai_board& board, size_t last_move, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept {
    if (status(board, last_move))
      return {-100, no_column};

    if (remaining == 0)
      return {0, no_column};

    // Possible values for maxv are:
    // -1 - loss
    //  0 - a tie
    //  1 - win
    //
    // We're initially setting it to -2 as worse than the worst case:
    int_fast8_t maxv = std::numeric_limits<int_fast8_t>::min();
    int_fast8_t column = no_column;

    for (int_fast8_t c = 0; c < static_cast<int_fast8_t>(board_width); ++c) {
      if (!column_available(board, c))
        continue;

      const auto offset = piece_offset(board, c);
      // std::cout << "Column: " << static_cast<size_t>(column) << " | Offset: " << offset << '\n';

      // On the empty field player Machine makes a move and calls Min
      // That's one branch of the game tree:
      piece_add(board, c, offset, Players::Machine);

      const auto m = min(board, offset, remaining - 1, alpha, beta).points;

      // Setting back the field to empty:
      piece_remove(board, c, offset);

      // Fixing the maxv value if needed:
      if (m > maxv) {
        maxv = m;
        column = c;

        alpha = std::max(alpha, maxv);
        if (alpha >= beta)
          break;
      }
    }

    return {maxv, column};
  }

  // Minimum is Players::Player
  ai_results min(
      ai_board& board, size_t last_move, int_fast8_t remaining, int_fast8_t alpha, int_fast8_t beta) noexcept {
    if (status(board, last_move))
      return {100, no_column};

    if (remaining == 0)
      return {0, no_column};

    // Possible values for minv are:
    // -1 - win
    //  0 - a tie
    //  1 - loss
    //
    // We're initially setting it to 2 as worse than the worst case:
    int_fast8_t minv = std::numeric_limits<int_fast8_t>::max();
    int_fast8_t column = no_column;

    for (int_fast8_t c = 0; c < static_cast<int_fast8_t>(board_width); ++c) {
      if (!column_available(board, c))
        continue;

      const auto offset = piece_offset(board, c);

      // On the empty field player Player makes a move and calls Max
      // That's one branch of the game tree:
      piece_add(board, c, offset, Players::Player);

      const auto m = max(board, offset, remaining - 1, alpha, beta).points;

      // Setting back the field to empty:
      piece_remove(board, c, offset);

      // Fixing the minv value if needed:
      if (m < minv) {
        minv = m;
        column = c;

        beta = std::min(beta, minv);
        if (alpha >= beta)
          break;
      }
    }

    return {minv, column};
  }

  void calculate_remaining(ai_board& board) noexcept {
    for (uint_fast8_t x = 0; x < board_width; ++x) {
      uint_fast8_t y = 0;
      while (y < board_height && board.cells[y * board_width + x] == Players::Unset)
        ++y;

      board.remaining[x] = y;
    }
  }

  int_fast8_t position(ai_board& board, int_fast8_t remaining, int_fast8_t maximum_depth) noexcept {
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
    if (remaining == 42)
      return 3;

    calculate_remaining(board);

    constexpr auto alpha = std::numeric_limits<int_fast8_t>::min();
    constexpr auto beta = std::numeric_limits<int_fast8_t>::max();

    // Process the best move for the AI.
    return max(board, undefined_last_move, std::min(remaining, maximum_depth), alpha, beta).position;
  }
} // namespace connect_four
