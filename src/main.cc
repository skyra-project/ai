#include <napi.h>
#include <array>
#include "utils/Players.hpp"
#include "games/TicTacToe.hpp"
#include "games/ConnectFour.hpp"

Napi::Number TicTacToeHandler(const Napi::CallbackInfo &info)
{
	auto env = info.Env();

	if (!info[0].IsTypedArray())
	{
		NAPI_THROW(Napi::TypeError::New(env, "data must be a typed array"), {});
	}

	const auto &v = info[0].As<Napi::Uint8Array>();
	if (v.ElementLength() != 9)
	{
		NAPI_THROW(Napi::TypeError::New(env, "data must have exactly 9 numbers"), {});
	}

	TicTacToe::ai_board matrix{};
	int remaining = 9;

	for (size_t i = 0; i < matrix.size(); ++i)
	{
		if ((matrix[i] = static_cast<Players>(v[i])) != Players::Unset)
			--remaining;
	}

	return Napi::Number::New(env, TicTacToe::position(matrix, remaining));
}

Napi::Number ConnectFourHandler(const Napi::CallbackInfo &info)
{
	auto env = info.Env();

	if (!info[0].IsTypedArray())
	{
		NAPI_THROW(Napi::TypeError::New(env, "data must be a typed array"), {});
	}

	const auto &v = info[0].As<Napi::Uint8Array>();
	if (v.ElementLength() != 42)
	{
		NAPI_THROW(Napi::TypeError::New(env, "data must have exactly 42 numbers"), {});
	}

	int_fast8_t maximum_depth;
	if (info[1].IsNumber())
		maximum_depth = static_cast<int_fast8_t>(info[1].As<Napi::Number>().Int32Value());
	else
		maximum_depth = 5;

	connect_four::ai_board board{};
	int_fast8_t remaining = 42;

	for (size_t i = 0; i < board.cells.size(); ++i)
	{
		if ((board.cells[i] = static_cast<Players>(v[i])) != Players::Unset)
			--remaining;
	}

	return Napi::Number::New(env, connect_four::position(board, remaining, maximum_depth));
}

Napi::Object Init(Napi::Env env, Napi::Object exports)
{
	exports.Set(Napi::String::New(env, "TicTacToe"),
				Napi::Function::New(env, TicTacToeHandler));
	exports.Set(Napi::String::New(env, "ConnectFour"),
				Napi::Function::New(env, ConnectFourHandler));
	return exports;
}

NODE_API_MODULE(addon, Init)
