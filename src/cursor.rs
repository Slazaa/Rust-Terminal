mod os;

use crate::utils::{CursorVisibility, Position};

pub fn get_position() -> Position {
	#[cfg(windows)]
	os::windows::get_position()
}

pub fn set_blinking(state: bool) {
	#[cfg(windows)]
	os::windows::set_blinking(state);
}

pub fn set_position(position: Position) {
	#[cfg(windows)]
	os::windows::set_position(position);
}

pub fn set_visibility(visibility: CursorVisibility) {
	#[cfg(windows)]
	os::windows::set_visibility(visibility);
}