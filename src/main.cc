#include "games/ConnectFour.hpp"
#include "games/TicTacToe.hpp"
#include "utils/Players.hpp"
#include <array>
#include <napi.h>

Napi::Number tic_tac_toe_handler(const Napi::CallbackInfo& info) {
  auto env = info.Env();

  if (!info[0].IsTypedArray()) {
    NAPI_THROW(Napi::TypeError::New(env, "data must be a typed array"), {});
  }

  const auto& v = info[0].As<Napi::Uint8Array>();
  if (v.ElementLength() != tic_tac_toe::board_cells) {
    NAPI_THROW(Napi::TypeError::New(env, "data must have exactly 9 numbers"), {});
  }

  tic_tac_toe::ai_board board{};
  int remaining = tic_tac_toe::board_cells;

  for (size_t i = 0; i < board.size(); ++i) {
    if ((board[i] = static_cast<Players>(v[i])) != Players::Unset)
      --remaining;
  }

  return Napi::Number::New(env, tic_tac_toe::position(board, remaining));
}

Napi::Number connect_four_handler(const Napi::CallbackInfo& info) {
  auto env = info.Env();

  if (!info[0].IsTypedArray()) {
    NAPI_THROW(Napi::TypeError::New(env, "data must be a typed array"), {});
  }

  const auto& v = info[0].As<Napi::Uint8Array>();
  if (v.ElementLength() != connect_four::board_cells) {
    NAPI_THROW(Napi::TypeError::New(env, "data must have exactly 42 numbers"), {});
  }

  int_fast8_t maximum_depth;
  if (info[1].IsNumber())
    maximum_depth = static_cast<int_fast8_t>(info[1].As<Napi::Number>().Int32Value());
  else
    maximum_depth = 5;

  connect_four::ai_board board{};
  int_fast8_t remaining = connect_four::board_cells;

  for (size_t i = 0; i < board.cells.size(); ++i) {
    if ((board.cells[i] = static_cast<Players>(v[i])) != Players::Unset)
      --remaining;
  }

  return Napi::Number::New(env, connect_four::position(board, remaining, maximum_depth));
}

Napi::Object Init(Napi::Env env, Napi::Object exports) {
  exports["ticTacToe"] = Napi::Function::New(env, tic_tac_toe_handler);
  exports["connectFour"] = Napi::Function::New(env, connect_four_handler);
  return exports;
}

NODE_API_MODULE(addon, Init)
