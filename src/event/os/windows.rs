use winapi::{
    shared::minwindef::{
        FALSE,
        TRUE
    },
	um::{
		consoleapi::{
            GetConsoleMode,
            SetConsoleMode,
            ReadConsoleInputW
        },
		processenv::GetStdHandle,
		winbase::STD_INPUT_HANDLE,
        wincon::{
            ENABLE_MOUSE_INPUT,
            ENABLE_WINDOW_INPUT,
        },
	    wincontypes::{
            DOUBLE_CLICK,
            FROM_LEFT_1ST_BUTTON_PRESSED,
            KEY_EVENT,
            MOUSE_EVENT,
            RIGHTMOST_BUTTON_PRESSED
        }
    }
};

use crate::event::{self, Event, MouseButton, MouseEvent};

pub fn read() -> Result<Event, String> {
	unsafe {
		let h_console = GetStdHandle(STD_INPUT_HANDLE);
		let mut old_mode = 0;

		if GetConsoleMode(h_console, &mut old_mode) == FALSE {
			return Err("Failed getting console mode".to_owned());
		} 

        if SetConsoleMode(h_console, ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT) == FALSE {
            return Err("Failed setting console mode".to_owned());
        }

        let mut input = std::mem::zeroed();
        let mut num_event_read = 0;

        if ReadConsoleInputW(h_console, &mut input, 1, &mut num_event_read) == FALSE {
            return Err("Failed peeking console input: {}".to_owned()); 
        }

        SetConsoleMode(h_console, old_mode);

        if num_event_read != 0 {
            let mouse_button = match input.Event.MouseEvent().dwButtonState {
                FROM_LEFT_1ST_BUTTON_PRESSED => MouseButton::Left,
                RIGHTMOST_BUTTON_PRESSED => MouseButton::Right,
                _ => MouseButton::Unknown
            };

            match input.EventType {
                KEY_EVENT => return Ok(Event::Key {
                    key: char::from_u32(*input.Event.KeyEvent().uChar.UnicodeChar() as u32).unwrap(),
                    down: if input.Event.KeyEvent().bKeyDown == TRUE { true } else { false }
                }),
                MOUSE_EVENT => return Ok(Event::Mouse(
                    match input.Event.MouseEvent().dwEventFlags {
                        0 => MouseEvent::Pressed(mouse_button),
                        DOUBLE_CLICK => MouseEvent::DoubleClick(mouse_button),
                        _ => MouseEvent::Unknown
                    }
                )),
                _ => return Ok(Event::Unknown) 
            }
        }
	}

    Err("No event were read".to_owned())
}
