
use crate::*;

pub struct Camera {
	pub center: (f32, f32),
	pub zoom: (f32, f32)
}

impl Camera {
	pub fn to_buffer(&self) -> [f32; 4] {
		[self.center.0, self.center.1, self.zoom.0, self.zoom.1]
	}
}