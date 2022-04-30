#[derive(Debug)]
pub struct Position {
	pub x: u32,
	pub y: u32
}

impl Position {
	pub fn new(x: u32, y: u32) -> Self {
		Self {
			x,
			y
		}
	}
}

#[derive(Debug)]
pub struct Size {
	pub width: u32,
	pub height: u32
}

impl Size {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height
		}
	}
}