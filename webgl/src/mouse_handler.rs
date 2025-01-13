use crate::*;

pub struct MouseHandler {
	wp: Webpage
}

#[derive(Clone)]
pub struct Webpage {
	pub document: Rc<RefCell<Document>>,
	pub context: Rc<RefCell<WebGl2RenderingContext>>,
	pub program: Rc<RefCell<WebGlProgram>>,
	pub window: Rc<RefCell<Window>>
}

impl MouseHandler {
	pub fn handle(&self, event:MouseEvent) {
		let context = self.wp.context.clone();
		let context = context.borrow_mut();
		let program = self.wp.program.clone();
		let program = program.borrow_mut();
		let window = self.wp.window.clone();
		let window = window.borrow_mut();
		alert(&format!("Mouse {}!", &event.offset_x().to_string()));
	}

	pub fn new(document: &web_sys::Document, wp:Webpage) -> Result<Rc<RefCell<Self>>, JsValue> {

		let mouse_handler_cell = Rc::new(RefCell::new(Self { wp: wp.clone() }));
		let mouse_handler = mouse_handler_cell.clone(); 
		let mouse_handler = mouse_handler.borrow_mut();
		let mouse_handler_cell_handler = mouse_handler_cell.clone();
		let handler = Closure::<dyn FnMut(_)>::new(
        	move |event: MouseEvent| {
               mouse_handler_cell_handler.borrow_mut().handle(event)
	        }
	    );

	    wp.document.clone().borrow().add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;

	    //document.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
	    handler.forget();
	    Ok(mouse_handler_cell.clone())
	}  


}

