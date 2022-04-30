mod os;

use crate::utils::Size;

pub fn clear() {
	#[cfg(windows)]
	os::windows::clear();
}

pub fn get_size() -> Size {
	#[cfg(windows)]
	os::windows::get_size()
}

pub fn get_title() -> String {
	#[cfg(windows)]
	os::windows::get_title()
}

pub fn set_title(title: &str) {
	#[cfg(windows)]
	os::windows::set_title(title);
}