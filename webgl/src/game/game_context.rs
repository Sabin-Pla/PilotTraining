use crate::*;

pub struct GameContext {
	pub wp: Webpage,
	pub track_shader: WebGlProgram,
	pub interface_shader: WebGlProgram,
	pub input_handler: InputHandler,
	pub aspect_ratio: (f32, f32),
	pub screen_ratio: f32,
	pub super_sampling_ratio: f32,
	pub camera: Rc<RefCell<Camera>>
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
	    let document = self.wp.document.clone();
	    let document = document.borrow();
	    let canvas = document.get_element_by_id("canvas").unwrap();
	        let canvas:HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();


		let (pixels_x, pixels_y) = get_screen_res(&*window, self.screen_ratio, self.aspect_ratio);
		let (mut pixels_x, pixels_y) = (
			canvas.style().get_property_value("width").unwrap(), 
			canvas.style().get_property_value("height").unwrap());
		
		let (pixels_x, pixels_y) = match pixels_x.ends_with("px") {
			true => { 

				let pixels_x = pixels_x.split_at(pixels_x.len()-2).0.to_string();
				let pixels_x = pixels_x.parse::<f32>().expect(&pixels_x);
				(pixels_x, pixels_x * self.aspect_ratio.1 / self.aspect_ratio.0)
			},
			false => { 
				if !pixels_y.ends_with("px") {
					panic!("no width or height property provided");
				}
				let pixels_y = pixels_y.split_at(pixels_y.len()-2).0.to_string();
				let pixels_y = pixels_y.parse::<f32>().expect(&pixels_x);
				(pixels_y * self.aspect_ratio.0 / self.aspect_ratio.1, pixels_y)
			}
		};

		(pixels_x, pixels_y)
	}

}

