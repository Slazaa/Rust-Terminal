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
			CONSOLE_CURSOR_INFO,
			CONSOLE_SCREEN_BUFFER_INFO,
			GetConsoleCursorInfo,
			GetConsoleScreenBufferInfo,
			SetConsoleCursorInfo,
			SetConsoleCursorPosition
		},
		wincontypes::COORD
	}
};

use crate::utils::Position;

pub fn get_pos() -> Result<Position, String> {
	unsafe {
		let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();

		if GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut csbi) == FALSE {
			return Err(format!("GetCursorPos failed with error code {}", GetLastError()));
		}
		
		Ok(Position::new(csbi.dwCursorPosition.X as u32, csbi.dwCursorPosition.Y as u32))
	}
}

pub fn is_visible() -> Result<bool, String> {
	unsafe {
		let mut cci: CONSOLE_CURSOR_INFO = std::mem::zeroed();
		
		if GetConsoleCursorInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut cci) == FALSE {
			return Err(format!("GetConsoleCursorInfo failed with error code {}", GetLastError()));
		}

		if cci.bVisible == TRUE {
			return Ok(true);
		}

		Ok(false)
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

pub fn set_visible(visible: bool) -> Result<(), String> {
	unsafe {
		let h_console = GetStdHandle(STD_OUTPUT_HANDLE);
		let mut cci: CONSOLE_CURSOR_INFO = std::mem::zeroed();

		if GetConsoleCursorInfo(h_console, &mut cci) == FALSE {
			return Err(format!("GetConsoleCursorInfo failed with error code {}", GetLastError()));
		}

		cci.bVisible = if visible { TRUE } else { FALSE };

		if SetConsoleCursorInfo(h_console, &cci) == FALSE {
			return Err(format!("SetConsoleCursorInfo failed with error code {}", GetLastError()));
		}

		Ok(())
	}
}