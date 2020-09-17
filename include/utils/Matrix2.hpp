#pragma once

#include <array>

template <class T, size_t Rows, size_t Columns>
class Matrix2
{
	std::array<T, Rows * Columns> data_{};

public:
	Matrix2() noexcept = default;
	Matrix2(T *array) noexcept
	{
		data_.data() = array;
	}

	constexpr inline size_t rows() const noexcept
	{
		return Rows;
	}

	constexpr inline size_t columns() const noexcept
	{
		return Columns;
	}

	constexpr inline size_t size() const noexcept
	{
		return Rows * Columns;
	}

	inline bool contains(T value) const noexcept
	{
		for (const auto &v : data_)
		{
			if (value == v)
				return true;
		}

		return false;
	}

	inline T get(size_t i) const noexcept
	{
		return data_[i];
	}

	inline T get(size_t x, size_t y) const noexcept
	{
		return data_[x + y * Columns];
	}

	inline void set(size_t i, T value) noexcept
	{
		data_[i] = value;
	}

	inline void set(size_t x, size_t y, T value) noexcept
	{
		data_[x + y * Columns] = value;
	}
};
