#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod games {
	pub mod connect_four;
	pub mod tic_tac_toe;
}

use games::tic_tac_toe;
use napi::bindgen_prelude::{ToNapiValue, Uint8Array};
use napi::Error;

#[napi]
#[derive(Debug, PartialEq)]
pub enum Players {
	Unset,
	Player,
	Machine,
}

impl TryFrom<u8> for Players {
	type Error = &'static str;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Players::Unset),
			1 => Ok(Players::Player),
			2 => Ok(Players::Machine),
			_ => Err("Players only accepts 0, 1, or 2!"),
		}
	}
}

pub const U_INVALID_INDEX: usize = 255;

#[napi]
pub const INVALID_INDEX: i64 = U_INVALID_INDEX as i64;

#[napi(js_name = "ticTacToe")]
pub fn tic_tac_toe_handler(v: Uint8Array) -> Result<i64, Error> {
	let input = v.to_vec();
	if input.len() != tic_tac_toe::BOARD_CELLS {
		return Err(Error::from_reason("data must have exactly 9 numbers"));
	}

	let mut cells: tic_tac_toe::AiCells = [Players::Unset; tic_tac_toe::BOARD_CELLS];
	let mut remaining: u8 = tic_tac_toe::BOARD_CELLS.try_into().unwrap();
	for i in 0..tic_tac_toe::BOARD_CELLS {
		cells[i] = Players::try_from(v[i]).map_err(Error::from_reason)?;
		if cells[i] != Players::Unset {
			remaining -= 1;
		}
	}

	let mut board = tic_tac_toe::AiBoard::new(cells);

	Ok(board.get_best_move(remaining).try_into().unwrap())
}

#[macro_export]
macro_rules! many_eq {
	($x:expr, $y:expr $(,$rest:expr)*) => {
		$x == $y && many_eq!($y $(,$rest)*)
	};
	($x:expr) => { true };
}
