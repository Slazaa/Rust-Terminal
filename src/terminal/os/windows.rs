use winapi::{
	shared::minwindef::{
		FALSE,
		TRUE
	},
	um::{
		errhandlingapi::GetLastError,
		handleapi::INVALID_HANDLE_VALUE,
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

use crate::utils::{Size, to_wstring};

const MAX_TITLE_SIZE: usize = 100;

pub fn clear() -> Result<(), String> {
	unsafe {
		let h_console = GetStdHandle(STD_OUTPUT_HANDLE);

		if h_console == INVALID_HANDLE_VALUE {
			return Err(format!("GetStdHandle failed with error code {}", GetLastError()));
		}

		let coord_screen: COORD = std::mem::zeroed();
		let mut c_char_written = 0;
		let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
		let dw_con_size;

		if GetConsoleScreenBufferInfo(h_console, &mut csbi) == FALSE {
			return Err(format!("GetConsoleScreenBufferInfo failed with error code {}", GetLastError()));
		}

		dw_con_size = csbi.dwSize.X as u32 * csbi.dwSize.Y as u32;

		if FillConsoleOutputCharacterA(h_console, ' ' as i8, dw_con_size, coord_screen, &mut c_char_written) == FALSE {
			return Err(format!("FillConsoleOutputCharacterA failed with error code {}", GetLastError()));
		}

		if GetConsoleScreenBufferInfo(h_console, &mut csbi) == FALSE {
			return Err(format!("GetConsoleScreenBufferInfo failed with error code {}", GetLastError()));
		}

		if FillConsoleOutputAttribute(h_console, csbi.wAttributes, dw_con_size, coord_screen, &mut c_char_written) == FALSE {
			return Err(format!("FillConsoleOutputAttribute failed with error code {}", GetLastError()));
		}

		if SetConsoleCursorPosition(h_console, coord_screen) == FALSE {
			return Err(format!("SetConsoleCursorPosition failed with error code {}", GetLastError()));
		}

		Ok(())
	}
}

pub fn get_size() -> Result<Size, String> {
	unsafe {
		let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
		
		if GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut csbi) == FALSE {
			return Err(format!("GetConsoleScreenBufferInfo failed with error code {}", GetLastError()));
		}
		
		Ok(Size::new(csbi.dwSize.X as u32, csbi.dwSize.Y as u32))
	}
}

pub fn get_title() -> Result<String, String> {
	unsafe {
		let mut title = vec![0; MAX_TITLE_SIZE];
		
		if GetConsoleTitleW(title.as_mut_ptr(), MAX_TITLE_SIZE as u32) == 0 {
			return Err(format!("GetConsoleTitleW failed with error code {}", GetLastError()));
		}
		
		Ok(String::from_utf16(&title).unwrap())
	}
}

pub fn set_size(size: Size) -> Result<(), String> {
	unsafe {
		let mut rect: SMALL_RECT = std::mem::zeroed();

		rect.Right = (size.width - 1) as i16;
		rect.Bottom = (size.height - 1) as i16;

		if SetConsoleWindowInfo(GetStdHandle(STD_OUTPUT_HANDLE), TRUE, &rect) == FALSE {
			return Err(format!("SetConsoleWindowInfo failed with error code {}", GetLastError()));
		}

		Ok(())
	}
}

pub fn set_title(title: &str) -> Result<(), String> {
	unsafe {
		if SetConsoleTitleW(to_wstring(title).as_ptr()) == FALSE {
			return Err(format!("SetConsoleTitleW failed with error code {}", GetLastError()));
		}

		Ok(())
	}
}