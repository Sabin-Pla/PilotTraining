use crate::*;

pub struct KeyHandler {
	wp: Webpage
}

impl KeyHandler {
	pub fn handle(&self, event: KeyboardEvent) {

		match event.code().as_str() {
			"KeyP" => { panic!("P is for panic!") },
			_ => { alert("must press w") }
		}
	}

	pub fn new(wp: Webpage) -> Result<Rc<RefCell<Self>>, JsValue> {

		let key_handler_cell = Rc::new(RefCell::new(Self { wp: wp.clone() }));
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

