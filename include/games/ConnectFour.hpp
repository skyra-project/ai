#pragma once

#include <cstdint>
#include <array>
#include "utils/Players.hpp"

namespace ConnectFour
{
	static constexpr uint_fast32_t board_width = 7U;
	static constexpr uint_fast32_t board_height = 6U;

	static constexpr uint_fast32_t board_min = 0U;
	static constexpr uint_fast32_t board_max = board_width * board_height;
	typedef std::array<Players, board_max> ai_board;

	int_fast8_t position(ai_board &board, int_fast8_t remaining) noexcept;
} // namespace ConnectFour
