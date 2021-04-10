cmake_minimum_required(VERSION 3.16.0)
project(skyra-ai VERSION 1.0.2)

set (CMAKE_CXX_STANDARD 17)
set (test_SOURCES tests/Connect4.cc src/games/ConnectFour.cc src/games/TicTacToe.cc deps/googletest/googletest/src/gtest_main.cc deps/googletest/googletest/src/gtest-all.cc)
add_executable(skyra_ai_tests ${test_SOURCES})
target_include_directories(skyra_ai_tests PRIVATE include deps/googletest/googletest/include deps/googletest/googletest)

# Link pthread on Linux
target_link_libraries(skyra_ai_tests "$<$<CXX_COMPILER_ID:GNU>:-pthread>$<$<CXX_COMPILER_ID:Clang>:-pthreads>")
