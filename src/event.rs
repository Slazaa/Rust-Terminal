mod os;

#[derive(Debug)]
pub enum MouseButton {
	Left,
	Right,
    Unknown
}

#[derive(Debug)]
pub enum MouseEvent {
	Pressed(MouseButton),
	DoubleClick(MouseButton),
	Moved,
	Wheel,
    Unknown
}

#[derive(Debug)]
pub enum Event {
	Key {
        key: char,
        down: bool
    },
	Mouse(MouseEvent),
    Unknown
}

pub fn read() -> Result<Event, String> {
    #[cfg(unix)]
    return os::unix::read();

    #[cfg(windows)]
    return os::windows::read();
}
