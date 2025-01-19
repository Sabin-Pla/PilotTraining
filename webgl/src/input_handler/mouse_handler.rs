use crate::*;

pub struct MouseHandler {
	wp: Webpage,
	buffer: MouseBuffer
}


impl MouseHandler {

	pub fn handle_release(&mut self, event: MouseEvent) {
		self.buffer.is_held = false;
		self.handle_move(event);
	}

	pub fn handle_press(&mut self, event: MouseEvent) {
		self.buffer.is_held = true;
		self.handle_move(event);
	}

	pub fn handle_move(&mut self, event: MouseEvent) {
		self.buffer.current_pos = (event.offset_x() as f32, event.offset_y() as f32);
	}


	pub fn new(wp:Webpage) -> Result<Rc<RefCell<Self>>, JsValue> {
		make_mouse_handler(wp)	
	}

	pub fn mouse_clipspace_coords(&self, x_resolution: f32, y_resolution: f32) -> (f32, f32) {
		let (x, y) = self.buffer.current_pos;
		(x / x_resolution, y / y_resolution)
	}


	
}



fn make_mouse_handler(wp:Webpage) -> Result<Rc<RefCell<MouseHandler>>, JsValue> {
	let mouse_handler_cell = Rc::new(RefCell::new(MouseHandler { wp: wp.clone(), buffer: Default::default() }));
	let press_handler = mouse_handler_cell.clone(); 
	let release_handler = mouse_handler_cell.clone(); 
	let move_handler = mouse_handler_cell.clone(); 
	let document = wp.document.clone();
	let document = document.borrow();
	let canvas = document.get_element_by_id("canvas").unwrap();
    let body: HtmlElement = canvas.parent_element().unwrap().dyn_into::<HtmlElement>()?;
    let canvas:HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;


	let press_handler = Closure::<dyn FnMut(_)>::new(
		move |event: MouseEvent| {
			press_handler.borrow_mut().handle_press(event)
		}
	);

	let release_handler = Closure::<dyn FnMut(_)>::new(
		move |event: MouseEvent| {
			release_handler.borrow_mut().handle_press(event)
		}
	);

	let move_handler = Closure::<dyn FnMut(_)>::new(
		move |event: MouseEvent| {
			move_handler.borrow_mut().handle_move(event)
		}
	);


	canvas.set_onmousedown(Some(press_handler.as_ref().unchecked_ref()));
	canvas.set_onmouseup(Some(release_handler.as_ref().unchecked_ref()));
	canvas.set_onmousemove(Some(move_handler.as_ref().unchecked_ref()));
	press_handler.forget();
	release_handler.forget();
	move_handler.forget();
	Ok(mouse_handler_cell.clone())
}