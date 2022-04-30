#[derive(Debug)]
pub struct Size {
	width: u32,
	height: u32
}

impl Size {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height
		}
	}

	pub fn width(&self) -> &u32 {
		&self.width
	}

	pub fn height(&self) -> &u32 {
		&self.height
	}

	pub fn width_mut(&mut self) -> &mut u32 {
		&mut self.width
	}

	pub fn height_mut(&mut self) -> &mut u32 {
		&mut self.height
	}
}