#pragma once

#include <cstdint>
#include <array>
#include "utils/Players.hpp"

namespace TicTacToe
{
	typedef std::array<Players, 9> ai_board;

	int_fast8_t position(ai_board &board, int_fast8_t remaining) noexcept;
} // namespace TicTacToe
