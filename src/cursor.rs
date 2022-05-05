mod os;

use crate::utils::Position;

pub fn get_pos() -> Result<Position, String> {
	#[cfg(unix)]
	return os::unix::get_pos();

	#[cfg(windows)]
	return os::windows::get_pos();
}

pub fn is_visible() -> Result<bool, String> {
	#[cfg(unix)]
	return os::unix::is_visible();

	#[cfg(windows)]
	return os::windows::is_visible();
}

pub fn set_pos(pos: Position) -> Result<(), String> {
	#[cfg(unix)]
	return os::unix::set_pos(pos);

	#[cfg(windows)]
	return os::windows::set_pos(pos);
}

pub fn set_visible(visible: bool) -> Result<(), String> {
	#[cfg(unix)]
	return os::unix::set_visible(visible);
	
	#[cfg(windows)]
	return os::windows::set_visible(visible);
}