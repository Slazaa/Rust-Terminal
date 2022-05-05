mod os;

use crate::utils::Size;

pub fn clear() -> Result<(), String> {
	#[cfg(unix)]
	return os::unix::clear();

	#[cfg(windows)]
	return os::windows::clear();
}

pub fn get_size() -> Result<Size, String> {
	#[cfg(unix)]
	return os::unix::get_size();

	#[cfg(windows)]
	return os::windows::get_size();
}

pub fn get_title() -> Result<String, String> {
	#[cfg(unix)]
	return os::unix::get_title();

	#[cfg(windows)]
	return os::windows::get_title();
}

pub fn set_size(size: Size) -> Result<(), String> {
	#[cfg(unix)]
	return os::unix::set_size(size);

	#[cfg(windows)]
	return os::windows::set_size(size);
}

pub fn set_title(title: &str) -> Result<(), String> {
	#[cfg(unix)]
	return os::unix::set_title(title);
	
	#[cfg(windows)]
	return os::windows::set_title(title);
}