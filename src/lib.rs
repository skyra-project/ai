#![deny(clippy::all)]
#![feature(portable_simd)]
#![feature(iter_map_windows)]

use std::mem::transmute;

#[macro_use]
extern crate napi_derive;

mod games {
	pub mod connect_four;
	pub mod tic_tac_toe;
}

#[napi]
#[derive(Debug, PartialEq)]
pub enum Player {
	Unset,
	Human,
	Machine,
}

impl From<Player> for i8 {
	fn from(value: Player) -> Self {
		unsafe { transmute(value) }
	}
}

impl TryFrom<u8> for Player {
	type Error = &'static str;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Player::Unset),
			1 => Ok(Player::Human),
			2 => Ok(Player::Machine),
			_ => Err("Player only accepts 0, 1, or 2!"),
		}
	}
}

pub const U_INVALID_INDEX: usize = 255;

#[napi]
pub const INVALID_INDEX: i64 = U_INVALID_INDEX as i64;

#[macro_export]
macro_rules! many_eq {
	($x:expr, $y:expr $(,$rest:expr)* $(,)?) => {
		$x == $y && many_eq!($y $(,$rest)*)
	};
	($x:expr) => { true };
}

#[macro_export]
macro_rules! isize_to_usize {
	($input:expr, $max:expr) => {
		(if $input < 0 {
			Err(Error::from_reason(concat!(stringify!($input), " must be a positive number")))
		} else if $input >= ($max as i32) {
			Err(Error::from_reason(format!(concat!(stringify!($input), " must be lower than {}"), $max)))
		} else {
			Ok($input as usize)
		})
	};
}

#[macro_export]
macro_rules! napi_assert {
	($input:expr) => {
		(if !$input {
			return Err(Error::from_reason(concat!("The assertion [", stringify!($input), "] failed")));
		})
	};
}
