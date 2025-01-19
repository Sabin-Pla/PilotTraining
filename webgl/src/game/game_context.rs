use crate::*;

pub struct GameContext {
	pub wp: Webpage,
	pub track_shader: WebGlProgram,
	pub interface_shader: WebGlProgram,
	pub input_handler: InputHandler,
	pub aspect_ratio: (f32, f32),
	pub screen_ratio: f32,
	pub super_sampling_ratio: f32,
	pub camera: Camera
}

impl GameContext {
	pub fn display_resolution(&self) -> (f32, f32) {
		let window = self.wp.window.clone();
		let window = window.borrow();
		let (width, height) = get_raster_res(&window, 
			self.super_sampling_ratio, 
        	self.screen_ratio,
        	self.aspect_ratio);
	    (width as f32, height as f32)
	}

	pub fn mouse_internal_resolution(&self) -> (f32, f32) {
		let window = self.wp.window.clone();
	    let window = window.borrow();
		let (pixels_x, pixels_y) = get_screen_res(&*window, self.screen_ratio, self.aspect_ratio);
		(pixels_x as f32, pixels_y as f32)
	}

}

