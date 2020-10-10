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

	ConnectFour::ai_board matrix{};
	int remaining = 42;

	for (size_t i = 0; i < matrix.size(); ++i)
	{
		if ((matrix[i] = static_cast<Players>(v[i])) != Players::Unset)
			--remaining;
	}

	return Napi::Number::New(env, ConnectFour::position(matrix, remaining));
}

Napi::Object Init(Napi::Env env, Napi::Object exports)
{
	exports.Set(Napi::String::New(env, "ticTacToe"),
				Napi::Function::New(env, TicTacToeHandler));

	exports.Set(Napi::String::New(env, "connectFour"),
				Napi::Function::New(env, ConnectFourHandler));
	return exports;
}

NODE_API_MODULE(addon, Init)
