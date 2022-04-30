mod os;

use crate::utils::{CursorVisibility, Position};

pub fn get_pos() -> Resul<Position, String> {
	#[cfg(windows)]
	return os::windows::get_pos();

	#[cfg(unix)]
	return os::unix::get_pos();
}

pub fn is_visible() -> Result<bool, String> {
	#[cfg(windows)]
	return os::windows::is_visible();

	#[cfg(unix)]
	return os::unix::is_visible();
}

pub fn set_pos(pos: Position) -> Result<(), String> {
	#[cfg(windows)]
	return os::windows::set_pos(pos);

	#[cfg(unix)]
	return os::unix::set_pos(pos);
}

pub fn set_visibility(visible: bool) -> Result<(), String> {
	#[cfg(windows)]
	return os::windows::set_visibility(visibility);

	#[cfg(unix)]
	return os::unix::set_visibility(visibility);
}