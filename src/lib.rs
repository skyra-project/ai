#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod games;

use games::{connect_four, tic_tac_toe};
use napi::bindgen_prelude::Uint8Array;
use napi::Error;

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[napi(js_name = "connectFour")]
pub fn connect_four_handler(v: Uint8Array, maximum_depth: Option<u32>) -> Result<i64, Error> {
	let input = v.to_vec();
	if input.len() != connect_four::BOARD_CELLS {
		return Err(Error::from_reason("data must have exactly 42 numbers"));
	}

	let mut cells: connect_four::AiCells = [Players::Unset; connect_four::BOARD_CELLS];
	let mut remaining: u8 = connect_four::BOARD_CELLS.try_into().unwrap();
	for i in 0..connect_four::BOARD_CELLS {
		cells[i] = Players::try_from(v[i]).map_err(Error::from_reason)?;
		if cells[i] != Players::Unset {
			remaining -= 1;
		}
	}

	let mut board = connect_four::AiBoard::new(cells);

	Ok(board.position(remaining, maximum_depth.unwrap_or(5).try_into().unwrap()).try_into().unwrap())
}

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

	Ok(board.position(remaining).try_into().unwrap())
}
