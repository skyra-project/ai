#include <napi.h>
#include <array>
#include "utils/Players.hpp"
#include "games/TicTacToe.hpp"

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

Napi::Object Init(Napi::Env env, Napi::Object exports)
{
	exports.Set(Napi::String::New(env, "TicTacToe"),
				Napi::Function::New(env, TicTacToeHandler));
	return exports;
}

NODE_API_MODULE(addon, Init)
