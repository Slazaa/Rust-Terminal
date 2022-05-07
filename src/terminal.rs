mod os;

use crate::utils::Size;

pub fn clear() {
	#[cfg(unix)]
	return os::unix::clear();

	#[cfg(windows)]
	return os::windows::clear();
}

pub fn get_size() -> Size {
	#[cfg(unix)]
	return os::unix::get_size();

	#[cfg(windows)]
	return os::windows::get_size();
}

pub fn get_title() -> String {
	#[cfg(unix)]
	return os::unix::get_title();

	#[cfg(windows)]
	return os::windows::get_title();
}

pub fn set_size(size: Size) {
	#[cfg(unix)]
	return os::unix::set_size(size);

	#[cfg(windows)]
	return os::windows::set_size(size);
}

pub fn set_title(title: &str) {
	#[cfg(unix)]
	return os::unix::set_title(title);
	
	#[cfg(windows)]
	return os::windows::set_title(title);
}