#[derive(Copy, Clone)]
pub struct MouseBuffer {
	pub is_held: bool,
	pub current_pos: (f32, f32)
}


impl Default for MouseBuffer {
	fn default() -> Self {
		Self {
			is_held: false,
			current_pos: (100.0, 100.0)
		}
	}
}
