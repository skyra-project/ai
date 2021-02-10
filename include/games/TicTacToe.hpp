#pragma once

#include "utils/Players.hpp"
#include <array>
#include <cstdint>

namespace tic_tac_toe {
  typedef std::array<Players, 9> ai_board;

  int_fast8_t position(ai_board& board, int_fast8_t remaining) noexcept;
} // namespace tic_tac_toe
