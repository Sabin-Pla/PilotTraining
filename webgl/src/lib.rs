#![allow(dead_code)]
#![allow(unconditional_recursion)]

use wasm_bindgen::prelude::*;
use web_sys::*;
use std::cell::RefCell;
use std::rc::Rc;
use core::default;
use core::ops::Deref;

mod input_handler;
mod interface;
mod webpage;
mod webgl_const;
mod camera;
mod renderer;
mod game;
mod stage;

use webpage::*;
use webgl_const::*;
use camera::*;
use input_handler::*;
pub use stage::*;
pub use interface::*;
use game::*;

pub use game::QuadraticBezier;

const GAME_ASPECT_X: f32  = 16.0;
const GAME_ASPECT_Y: f32  = 9.0;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn get_viewport_dim(window: &Window) -> (u32, u32) {
    let width = window.outer_width().unwrap().as_f64().unwrap() * 0.98;
    let height = window.outer_height().unwrap().as_f64().unwrap() * 0.98;
    (width as u32, height as u32)
}

fn handle_resize( 
        canvas: Rc<RefCell<HtmlCanvasElement>>,
        context: Rc<RefCell<WebGl2RenderingContext>>,
        window: Rc<RefCell<Window>>) {
    let window = window.borrow_mut();
    let canvas = canvas.borrow_mut();
    let context = &context.borrow_mut();
    let (width, height) = get_viewport_dim(&*window);
 //   canvas.set_width(width);
  //  canvas.set_height(height);
 // //  context.viewport(0, 0,  
 //       canvas.width().try_into().unwrap(),
  //      canvas.height().try_into().unwrap());
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    // panic!("FUCK"); now this will be visible in the browser console :)
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    init_panic_hook();
    
    let window = web_sys::window().expect("Failed to start WASM: window()");
    let window_cell = Rc::new(RefCell::new(window));
    let window = window_cell.clone();
    let window = window.borrow();
    let document = window.document().unwrap();
    let document_cell = Rc::new(RefCell::new(document));
    let document = document_cell.clone();
    let document = document.borrow();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let body: HtmlElement = canvas.parent_element().unwrap().dyn_into::<HtmlElement>()?;
    let canvas:HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let (width, height) = get_viewport_dim(&*window);
    canvas.set_width(width);
    canvas.set_height(height);

    body.style().set_property("margin", "0px")?;
    canvas.style().set_property("position", "relative")?;
    canvas.style().set_property("margin", "auto")?;

    canvas.style().set_property("width", "500px")?;
    canvas.style().set_property("display", "block")?;
    canvas.style().set_property("aspect-ratio", "16.0 / 9.0")?;


    let canvas_cell = Rc::new(RefCell::new(canvas));
    let canvas = canvas_cell.clone();
    let canvas = canvas.borrow_mut();
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;
    let context_cell = Rc::new(RefCell::new(context));
    let context = context_cell.clone();
    let context = &context.borrow();

    let wp = Webpage { 
        document: document_cell.clone(), 
        context: context_cell.clone(),
        window: window_cell.clone()
    };

    let input_handler = InputHandler::new(wp.clone())?;
    
    let resize_handler = Closure::<dyn FnMut()>::new(move || { 
        handle_resize(
            canvas_cell.clone(),
            context_cell.clone(),
            window_cell.clone())
        }
    );

    let base_shaders = renderer::initialize_base_shaders(context); 

    let track_shader =  renderer::link_program(&context, 
        &base_shaders[1].0, &base_shaders[1].1);
    let interface_shader = renderer::link_program(&context, 
        &base_shaders[0].0, &base_shaders[0].1);


    let game_context = GameContext {
        wp: wp.clone(),
        track_shader: track_shader?,
        interface_shader: interface_shader?,
        input_handler: input_handler,
        aspect_ratio: (GAME_ASPECT_X, GAME_ASPECT_Y),
        camera: Camera::default()
    };


    let mut game = Game::new();
    game.load_stage(&DemoStage {});

    start_game_loop(game_context, game);
    // window.set_onresize(Some(resize_handler.as_ref().unchecked_ref()));
    resize_handler.forget();
    
    Ok(())
}