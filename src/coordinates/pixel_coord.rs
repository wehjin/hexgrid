#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct PixelCoord {
	pub x: f32,
	pub y: f32,
}

impl PixelCoord {
	pub fn same_y(&self) -> (f32, f32) {
		(self.x, self.y)
	}
	pub fn flip_y(&self) -> (f32, f32) {
		(self.x, -self.y)
	}
}