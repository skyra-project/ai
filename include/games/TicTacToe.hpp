#pragma once

#include "utils/Players.hpp"
#include <array>
#include <cstdint>

namespace tic_tac_toe {
  constexpr uint_fast8_t board_width = 3;
  constexpr uint_fast8_t board_height = 3;
  constexpr uint_fast8_t board_cells = board_width * board_height;

  typedef std::array<Players, board_cells> ai_board;

  int_fast8_t position(ai_board& board, int_fast8_t remaining) noexcept;
} // namespace tic_tac_toe
