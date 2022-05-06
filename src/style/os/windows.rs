use winapi::um::{
	processenv::GetStdHandle, 
	winbase::STD_OUTPUT_HANDLE,
	wincon::{
		BACKGROUND_BLUE,
		BACKGROUND_GREEN,
		BACKGROUND_INTENSITY,
		BACKGROUND_RED,
		FOREGROUND_BLUE,
		FOREGROUND_GREEN,
		FOREGROUND_INTENSITY,
		FOREGROUND_RED,
		CONSOLE_SCREEN_BUFFER_INFOEX,
		GetConsoleScreenBufferInfoEx,
		SetConsoleScreenBufferInfoEx,
	}
};

use crate::utils::Color;

fn convert_base_to_color(color: u16, background: bool) -> Color {
	let mut ret = Color::DarkGrey;

	if (!background && color & FOREGROUND_BLUE != 0) || (background && color & BACKGROUND_BLUE != 0) {
		ret = Color::DarkBlue;
	}

	if (!background && color & FOREGROUND_GREEN != 0) || (background && color & BACKGROUND_GREEN != 0) {
		ret = match ret {
			Color::DarkBlue => Color::DarkCyan,
			_ => Color::DarkGreen
		};
	}

	if (!background && color & FOREGROUND_RED != 0) || (background && color & BACKGROUND_RED != 0) {
		ret = match ret {
			Color::DarkBlue => Color::DarkMagenta,
			Color::DarkCyan => Color::Grey,
			Color::DarkGreen => Color::DarkYellow,
			_ => Color::DarkRed
		};
	}

	if (!background && color & FOREGROUND_INTENSITY != 0) || (background && color & BACKGROUND_INTENSITY != 0) {
		ret = match ret {
			Color::DarkBlue => Color::Blue,
			Color::DarkCyan => Color::Cyan,
			Color::DarkGreen => Color::Green,
			Color::DarkMagenta => Color::Magenta,
			Color::Grey => Color::White,
			Color::DarkYellow => Color::Yellow,
			Color::DarkRed => Color::Red,
			_ => Color::DarkGrey
		};
	}

	ret
}

fn convert_color_to_base(color: Color, background: bool) -> u16 {
	let mut ret = match color {
		Color::Black => 0,
		Color::Blue => FOREGROUND_BLUE | FOREGROUND_INTENSITY,
		Color::Cyan => FOREGROUND_BLUE | FOREGROUND_GREEN | FOREGROUND_INTENSITY,
		Color::DarkBlue => FOREGROUND_BLUE,
		Color::DarkCyan => FOREGROUND_BLUE | FOREGROUND_GREEN,
		Color::DarkGreen => FOREGROUND_GREEN,
		Color::DarkGrey => FOREGROUND_INTENSITY,
		Color::DarkMagenta => FOREGROUND_BLUE | FOREGROUND_RED,
		Color::DarkRed => FOREGROUND_RED,
		Color::DarkYellow => FOREGROUND_GREEN | FOREGROUND_RED,
		Color::Green => FOREGROUND_GREEN | FOREGROUND_INTENSITY,
		Color::Grey => FOREGROUND_BLUE | FOREGROUND_GREEN | FOREGROUND_RED,
		Color::Magenta => FOREGROUND_BLUE | FOREGROUND_RED | FOREGROUND_INTENSITY,
		Color::Red => FOREGROUND_RED | FOREGROUND_INTENSITY,
		Color::White => FOREGROUND_BLUE | FOREGROUND_GREEN | FOREGROUND_RED | FOREGROUND_INTENSITY,
		Color::Yellow => FOREGROUND_GREEN | FOREGROUND_RED | FOREGROUND_INTENSITY
	};

	if background {
		ret <<= 4;
	}

	ret
}

pub fn get_background_color() -> Color {
	unsafe {
		let mut cbi: CONSOLE_SCREEN_BUFFER_INFOEX = std::mem::zeroed();
		cbi.cbSize = std::mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() as u32;

		GetConsoleScreenBufferInfoEx(GetStdHandle(STD_OUTPUT_HANDLE), &mut cbi);
		convert_base_to_color(cbi.wAttributes, true)
	}
}

pub fn get_foreground_color() -> Color {
	unsafe {
		let mut cbi: CONSOLE_SCREEN_BUFFER_INFOEX = std::mem::zeroed();
		cbi.cbSize = std::mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() as u32;

		GetConsoleScreenBufferInfoEx(GetStdHandle(STD_OUTPUT_HANDLE), &mut cbi);
		convert_base_to_color(cbi.wAttributes, false)
	}
}

pub fn set_background_color(color: Color) {
	unsafe {
		let h_console = GetStdHandle(STD_OUTPUT_HANDLE);
		let mut cbi: CONSOLE_SCREEN_BUFFER_INFOEX = std::mem::zeroed();
		cbi.cbSize = std::mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() as u32;
		
		GetConsoleScreenBufferInfoEx(h_console, &mut cbi);
		cbi.wAttributes = convert_color_to_base(color, true);
		SetConsoleScreenBufferInfoEx(h_console, &mut cbi);
	}
}

pub fn set_foreground_color(color: Color) {
	unsafe {
		let h_console = GetStdHandle(STD_OUTPUT_HANDLE);
		let mut cbi: CONSOLE_SCREEN_BUFFER_INFOEX = std::mem::zeroed();
		cbi.cbSize = std::mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() as u32;
		
		GetConsoleScreenBufferInfoEx(h_console, &mut cbi);
		cbi.wAttributes = convert_color_to_base(color, false);
		SetConsoleScreenBufferInfoEx(h_console, &mut cbi);
	}
}