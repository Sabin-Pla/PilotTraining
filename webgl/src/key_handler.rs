use crate::*;

pub struct KeyHandler {
	context: Rc<RefCell<WebGl2RenderingContext>>,
	program: Rc<RefCell<WebGlProgram>>
}

impl KeyHandler {
	pub fn handle(&self, event: KeyboardEvent) {
		match event.code().as_str() {
			"KeyW" => {
				let context = self.context.clone();
				let context = context.borrow_mut();
				let program = self.program.clone();
				let program = program.borrow_mut();
				match stage_program(&context) {
					Ok(program) => {
						context.use_program(Some(&program));
						stage::init(&context, &program);
					},
					Err(err) => alert(&format!("Error loading stage: {}", err))
				}
			},
			_ => { alert("must press w") }
		}
	}

	pub fn new(
			document: &web_sys::Document, 
			context: Rc<RefCell<WebGl2RenderingContext>>,
			program: Rc<RefCell<WebGlProgram>>) -> Result<Rc<RefCell<Self>>, JsValue> {

		let key_handler_cell = Rc::new(RefCell::new(Self { context, program }));
		let key_handler = key_handler_cell.clone();
		let key_handler = key_handler.borrow_mut();
		let key_handler_cell_handler = key_handler_cell.clone();
		let handler = Closure::<dyn FnMut(_)>::new(
        	move |event: KeyboardEvent| {
               key_handler_cell_handler.borrow_mut().handle(event)
	        }
	    );
	    document.add_event_listener_with_callback(
	        "keypress", handler.as_ref().unchecked_ref())?;
	    handler.forget();
	    Ok(key_handler_cell.clone())
	}  
}

