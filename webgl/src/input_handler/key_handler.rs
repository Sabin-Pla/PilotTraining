use crate::*;

pub struct KeyHandler {
	wp: Webpage,
	camera: Rc<RefCell<Camera>>
}

impl KeyHandler {
	pub fn handle(&self, event: KeyboardEvent) {
		let mut camera = self.camera.clone();
		let mut camera = camera.borrow_mut();

		match event.code().as_str() {
			"KeyW" => {
				camera.center.y += 0.1;
			},
			"KeyA" => {
				camera.center.x -= 0.1;
			},
			"KeyS" => {
				camera.center.y -= 0.1;
			},
			"KeyD" => {
				camera.center.x += 0.1;
			},
			_ => {}
		}
	}

	pub fn new(wp: Webpage, camera: Rc<RefCell<Camera>>) -> Result<Rc<RefCell<Self>>, JsValue> {

		let key_handler_cell = Rc::new(RefCell::new(Self { wp: wp.clone(), camera: camera.clone() }));
		let key_handler = key_handler_cell.clone();
		let key_handler = key_handler.borrow_mut();
		let key_handler_cell_handler = key_handler_cell.clone();

		let handler = Closure::<dyn FnMut(_)>::new(
        	move |event: KeyboardEvent| {
               key_handler_cell_handler.borrow_mut().handle(event)
	        }
	    );

	    let document = wp.document.borrow();
	    document.add_event_listener_with_callback(
	        "keypress", handler.as_ref().unchecked_ref())?;
	    handler.forget();
	    Ok(key_handler_cell.clone())
	}  
}

