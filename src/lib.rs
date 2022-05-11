#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod common;
mod games;

use common::players::Players;
use games::{connect_four, tic_tac_toe};
use napi::bindgen_prelude::Uint8Array;
use napi::Error;

#[napi]
pub const INVALID_INDEX: i64 = common::players::INVALID_INDEX as i64;

#[napi(js_name = "connectFour")]
pub fn connect_four_handler(v: Uint8Array, maximum_depth: Option<u32>) -> Result<i64, Error> {
	let input = v.to_vec();
	if input.len() != connect_four::BOARD_CELLS {
		return Err(Error::from_reason("data must have exactly 9 numbers"));
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

	let mut board: tic_tac_toe::AiBoard = [Players::Unset; tic_tac_toe::BOARD_CELLS];
	let mut remaining: u8 = tic_tac_toe::BOARD_CELLS.try_into().unwrap();
	for i in 0..tic_tac_toe::BOARD_CELLS {
		board[i] = Players::try_from(v[i]).map_err(Error::from_reason)?;
		if board[i] != Players::Unset {
			remaining -= 1;
		}
	}

	Ok(tic_tac_toe::position(&mut board, remaining).try_into().unwrap())
}
