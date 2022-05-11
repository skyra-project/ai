#[derive(PartialEq, Copy, Clone)]
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

pub const INVALID_INDEX: usize = i64::MAX as usize;
