
use crate::*;

pub struct Camera {
	pub center: FieldPosition,
	pub zoom: f32,
	pub aspect_ratio: (f32, f32)
}

impl Default for Camera {
	fn default() -> Self {
		Self {
			zoom: 1.0,
			center: Default::default(),
			aspect_ratio: (1.0, 1.0)
		}
	}
}

impl Camera {
	pub fn to_buffer(&self, aspect_ratio: (f32, f32), field_clip_offset: (f32, f32)) -> [f32; 8] {

		// convert center to clipspace
		// worldspace maps to clipspace 1 to 1 at 1x zoom
		let center = self.center.clipspace((1.0, 1.0), (0.0, 0.0));
		[center.0, center.1, self.zoom, self.zoom, aspect_ratio.0, aspect_ratio.1, 0.0, 0.0]
	}
}