mod key_handler;
mod mouse_handler;
mod mouse_buffer;

pub use mouse_buffer::*;
pub use key_handler::*;
pub use mouse_handler::*;

use crate::*; 

pub struct InputHandler {
	key_handler: Rc<RefCell<KeyHandler>>,
	mouse_handler: Rc<RefCell<MouseHandler>>,
	wp: Webpage
}

impl InputHandler {
	pub fn new(wp: Webpage) -> Result<Self, JsValue> {
		Ok(InputHandler {
			key_handler: KeyHandler::new(wp.clone())?,
			mouse_handler: MouseHandler::new(wp.clone())?,
			wp: wp
		})
	}

	pub fn mouse_clipspace_coords(&self, x_resolution: f32, y_resolution: f32) -> (f32, f32) {
		let mouse_handler = self.mouse_handler.borrow();
		mouse_handler.mouse_clipspace_coords(x_resolution, y_resolution)
	}
}