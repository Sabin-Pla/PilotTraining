use crate::*;


// Structure representing position of an object in worldspace
#[derive(Copy, Clone, Default)]
pub struct FieldPosition {
	pub x: f32,
	pub y: f32
}

impl FieldPosition {
	pub fn from(x: f32, y: f32) -> Self {
		Self {
			x,
			y
		}
	}

	pub fn clipspace(&self,
			aspect_ratio: (f32, f32), 
			field_clip_offset: (f32, f32)) -> (f32, f32) { 
		// gets the location of this coordinate in clipspace, assuming camera is at origin

		(
			((self.x) * (aspect_ratio.1 / aspect_ratio.0)) + field_clip_offset.0,
			((self.y) * (aspect_ratio.0 / aspect_ratio.1)) + field_clip_offset.1
		)
	}
}