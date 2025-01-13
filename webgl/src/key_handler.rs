use crate::*;

pub struct KeyHandler {
	wp: Webpage
}

impl KeyHandler {
	pub fn handle(&self, event: KeyboardEvent) {
		let context = self.wp.context.clone();
		let context = context.borrow_mut();
		let program = self.wp.program.clone();
		let program = program.borrow_mut();
		let window = self.wp.window.clone();
		let window = window.borrow_mut();

		match event.code().as_str() {
			"KeyW" => {
				match stage_program(&context) {
					Ok(program) => {
						context.use_program(Some(&program));
						stage::init(&context, &program, &window);
					},
					Err(err) => alert(&format!("Error loading stage: {}", err))
				}
			},
			"KeyB" => {
				match stage_program_bezier(&context) {
					Ok(program) => {
						context.use_program(Some(&program));
						stage::init_bezier(&context, &program, &window);
					},
					Err(err) => alert(&format!("Error loading stage: {}", err))
				}
			},
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

