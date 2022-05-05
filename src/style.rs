mod os;

use crate::utils::Color;

pub fn get_background_color() -> Result<Color, String> {
	#[cfg(unix)]
	return os::unix::get_background_color();

	#[cfg(windows)]
	return os::windows::get_background_color();
}

pub fn get_foreground_color() -> Result<Color, String> {
	#[cfg(unix)]
	return os::unix::get_foreground_color();

	#[cfg(windows)]
	return os::windows::get_foreground_color();
}

pub fn get_foreground_color() -> Result<Color, String> {
	#[cfg(unix)]
	return os::unix::get_foreground_color();
	
	#[cfg(windows)]
	return os::windows::get_foreground_color();
}