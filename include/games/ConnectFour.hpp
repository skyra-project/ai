#pragma once

#include <cstdint>
#include <array>
#include "utils/Players.hpp"

namespace connect_four
{
	constexpr uint_fast8_t board_width = 7;
	constexpr uint_fast8_t board_height = 6;
	constexpr uint_fast8_t board_cells = board_width * board_height;

	typedef std::array<Players, board_cells> ai_cells;
	typedef std::array<uint_fast8_t, board_width> ai_remaining;

	struct ai_board {
		ai_cells cells;
		ai_remaining remaining;
	};

	int_fast8_t position(ai_board &board, int_fast8_t remaining, int_fast8_t maximum_depth) noexcept;
} // namespace connect_four
