#include "gtest/gtest.h"
#include "games/ConnectFour.hpp"

TEST(Connect4, Stub) {
	connect_four::ai_board b{};
	EXPECT_EQ(connect_four::position(b, 42, 5), 3);
}
