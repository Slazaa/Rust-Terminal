mod os;

use crate::utils::Size;

pub fn clear() -> Result<(), String> {
	#[cfg(windows)]
	return os::windows::clear();

	#[cfg(unix)]
	return os::unix::clear();
}

pub fn get_size() -> Result<Size, String> {
	#[cfg(windows)]
	return os::windows::get_size();

	#[cfg(unix)]
	return os::unix::get_size();
}

pub fn get_title() -> Result<String, String> {
	#[cfg(windows)]
	return os::windows::get_title();

	#[cfg(unix)]
	return os::unix::get_title();
}

pub fn set_size(size: Size) -> Result<(), String> {
	#[cfg(windows)]
	return os::windows::set_size(size);
	
	#[cfg(unix)]
	return os::unix::set_size(size);
}

pub fn set_title(title: &str) -> Result<(), String> {
	#[cfg(windows)]
	return os::windows::set_title(title);

	#[cfg(unix)]
	return os::unix::set_title(title);
}