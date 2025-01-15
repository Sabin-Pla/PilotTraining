use crate::*;

pub struct GameContext {
	pub wp: Webpage,
	pub track_shader: WebGlProgram,
	pub interface_shader: WebGlProgram,
	pub input_handler: InputHandler,
	pub aspect_ratio: (f32, f32),
	pub camera: Camera
}

impl GameContext {
	pub fn internal_resolution(&self) -> (f32, f32) {
		let window = self.wp.window.clone();
		let window = window.borrow();
		let (width, height): (f32, f32) = (
	    	window.inner_width().unwrap().as_f64().unwrap() as f32, 
	    	window.inner_height().unwrap().as_f64().unwrap() as f32);
		(width, height)
	}
}

