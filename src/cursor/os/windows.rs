use winapi::{
	shared::minwindef::{
		FALSE,
		TRUE
	},
	um::{
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

pub fn get_pos() -> Position {
	unsafe {
		let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();

		GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut csbi);
		
		Position::new(csbi.dwCursorPosition.X as u32, csbi.dwCursorPosition.Y as u32)
	}
}

pub fn is_visible() -> bool {
	unsafe {
		let mut cci: CONSOLE_CURSOR_INFO = std::mem::zeroed();
		
		GetConsoleCursorInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut cci);

		if cci.bVisible == TRUE {
			return true;
		}

		false
	}
}

pub fn set_pos(pos: Position) {
	unsafe {
		SetConsoleCursorPosition(GetStdHandle(STD_OUTPUT_HANDLE), COORD { X: pos.x as i16, Y: pos.y as i16 });
	}
}

pub fn set_visible(visible: bool) {
	unsafe {
		let h_console = GetStdHandle(STD_OUTPUT_HANDLE);
		let mut cci: CONSOLE_CURSOR_INFO = std::mem::zeroed();

		GetConsoleCursorInfo(h_console, &mut cci);

		cci.bVisible = if visible { TRUE } else { FALSE };

		SetConsoleCursorInfo(h_console, &cci);
	}
}
