use winapi::{
	shared::minwindef::TRUE,
	um::{
		processenv::GetStdHandle,
		winbase::STD_OUTPUT_HANDLE,
		wincon::{
			CONSOLE_SCREEN_BUFFER_INFO,
			FillConsoleOutputAttribute,
			FillConsoleOutputCharacterA,
			GetConsoleScreenBufferInfo,
			GetConsoleTitleW,
			SetConsoleCursorPosition,
			SetConsoleTitleW,
			SetConsoleWindowInfo
		},
		wincontypes::{
			COORD,
			SMALL_RECT
		}
	}
};

use crate::utils::Size;

const MAX_TITLE_SIZE: usize = 100;

fn wide_null(string: &str) -> Vec<u16> {
	string.encode_utf16().chain(Some(0)).collect()
}

pub fn clear() {
	unsafe {
		let h_console = GetStdHandle(STD_OUTPUT_HANDLE);

		let coord_screen: COORD = std::mem::zeroed();
		let mut c_char_written = 0;
		let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
		let dw_con_size;

		GetConsoleScreenBufferInfo(h_console, &mut csbi);

		dw_con_size = csbi.dwSize.X as u32 * csbi.dwSize.Y as u32;

		FillConsoleOutputCharacterA(h_console, ' ' as i8, dw_con_size, coord_screen, &mut c_char_written);
		GetConsoleScreenBufferInfo(h_console, &mut csbi);
		FillConsoleOutputAttribute(h_console, csbi.wAttributes, dw_con_size, coord_screen, &mut c_char_written);
		SetConsoleCursorPosition(h_console, coord_screen);
	}
}

pub fn get_size() -> Size {
	unsafe {
		let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
		GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut csbi);
		return Size::new(csbi.dwSize.X as u32, csbi.dwSize.Y as u32);
	}
}

pub fn get_title() -> String {
	unsafe {
		let mut title = vec![0; MAX_TITLE_SIZE];
		GetConsoleTitleW(title.as_mut_ptr(), MAX_TITLE_SIZE as u32);
		return String::from_utf16(&title).unwrap();
	}
}

pub fn set_size(size: Size) {
	unsafe {
		let rect = SMALL_RECT { Left: 0, Top: 0, Right: (size.width - 1) as i16, Bottom: (size.height - 1) as i16 };
		SetConsoleWindowInfo(GetStdHandle(STD_OUTPUT_HANDLE), TRUE, &rect);
	}
}

pub fn set_title(title: &str) {
	unsafe {
		SetConsoleTitleW(wide_null(title).as_ptr());
	}
}