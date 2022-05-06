mod os;

use crate::utils::Color;

pub fn get_background_color() -> Color {
	#[cfg(unix)]
	return os::unix::get_background_color();

	#[cfg(windows)]
	return os::windows::get_background_color();
}

pub fn get_foreground_color() -> Color {
	#[cfg(unix)]
	return os::unix::get_foreground_color();

	#[cfg(windows)]
	return os::windows::get_foreground_color();
}

pub fn set_background_color(color: Color) {
	#[cfg(unix)]
	os::unix::set_background_color(color);

	#[cfg(windows)]
	os::windows::set_background_color(color);
}

pub fn set_foreground_color(color: Color) {
	#[cfg(unix)]
	os::unix::set_foreground_color(color);

	#[cfg(windows)]
	os::windows::set_foreground_color(color);
}