use crate::*;

#[derive(Clone)]
pub struct Webpage {
	pub document: Rc<RefCell<Document>>,
	pub context: Rc<RefCell<WebGl2RenderingContext>>,
	pub window: Rc<RefCell<Window>>
}