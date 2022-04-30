use winapi::{
	shared::minwindef::{
		FALSE,
		TRUE
	},
	um::{
		errhandlingapi::GetLastError,
		processenv::GetStdHandle,
		winbase::STD_OUTPUT_HANDLE,
		wincon::{
			PCONSOLE_CURSOR_INFO,
			GetConsoleCursorInfo,
			SetConsoleCursorInfo,
			SetConsoleCursorPosition
		},
		wincontypes::COORD,
		winuser::GetCursorPos
	}
};

use crate::utils::Position;

pub fn get_pos() -> Result<Position, String> {
	unsafe {
		let mut pos = std::mem::zeroed();
		
		if GetCursorPos(&mut pos) == FALSE {
			return Err(format!("GetCursorPos failed with error code {}", GetLastError()));
		}
		
		Ok(Position::new(pos.x as u32, pos.y as u32))
	}
}

pub fn is_visible() -> Result<bool, String> {
	unsafe {
		let cci: PCONSOLE_CURSOR_INFO = std::mem::zeroed();
		
		
		if GetConsoleCursorInfo(GetStdHandle(STD_OUTPUT_HANDLE), cci) == FALSE {
			return Err(format!("GetConsoleCursorInfo failed with error code {}", GetLastError()));
		}

		if (*cci).bVisible == TRUE {
			return Ok(CursorVisibility::Shown);
		}

		Ok(CursorVisibility::Hidden)
	}
}

pub fn set_pos(pos: Position) -> Result<(), String> {
	unsafe {
		if SetConsoleCursorPosition(GetStdHandle(STD_OUTPUT_HANDLE), COORD { X: pos.x as i16, Y: pos.y as i16 }) == FALSE {
			return Err(format!("SetConsoleCursorPosition failed with error code {}", GetLastError()));
		}

		Ok(())
	}
}

pub fn set_visibility(visible: bool) -> Result<(), String> {
	unsafe {
		let cci: PCONSOLE_CURSOR_INFO = std::mem::zeroed();

		match visibility {
			CursorVisibility::Shown => (*cci).bVisible = TRUE,
			CursorVisibility::Hidden => (*cci).bVisible = FALSE
		}

		if SetConsoleCursorInfo(GetStdHandle(STD_OUTPUT_HANDLE), cci) == FALSE {
			return Err(format!("SetConsoleCursorInfo failed with error code {}", GetLastError()));
		}

		Ok(())
	}
}