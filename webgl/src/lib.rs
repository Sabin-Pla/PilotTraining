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

const GAME_ASPECT_X: f32  = 2.0;
const GAME_ASPECT_Y: f32  = 1.0;
const DEFAULT_SUPER_SAMPLING_RATIO: f32 = 2.0; // render at twice the display res
const DEFAULT_SCREEN_RATIO: f32 = 0.75; // portion of screen space to take up

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub fn get_viewport_dim(window: &Window) -> (f32, f32) {
    let width = window.outer_width().unwrap().as_f64().unwrap();
    let height = window.outer_height().unwrap().as_f64().unwrap();
    (width as f32, height as f32)
}

fn adjust_aspect(dim: &mut (f32, f32), aspect_ratio: (f32, f32)) {
    if dim.0 > dim.1 {
        let preferred_width = dim.1 as f32 * aspect_ratio.0 / aspect_ratio.1;
        if preferred_width > dim.0 {
            // can't use this aspect ratio with this height.
            dim.1 = dim.0 as f32 * aspect_ratio.1 / aspect_ratio.0;
            return
        } else {
            // viewport is height-bound, keep height as-is and adjust width
            dim.0 = dim.1 as f32 * aspect_ratio.0 / aspect_ratio.1;
        }
    } else {
        let preferred_height = dim.0 as f32 * aspect_ratio.1 / aspect_ratio.0;
        if preferred_height > dim.1 {
            // can't use this aspect ratio with this width.
            dim.0 = dim.1 as f32 * aspect_ratio.0 / aspect_ratio.1;
            return
        } else {
            // viewport is width-bound, keep width as-is and adjust height
            dim.1 = dim.0 as f32 * aspect_ratio.1 / aspect_ratio.0;
        }
    }
}

fn get_raster_res(
        window: &Window, 
        super_sampling_ratio: f32, 
        screen_ratio: f32, 
        aspect_ratio: (f32, f32)) -> (u32, u32) {
    // gets the display dimensions as recognized by the rasterizer
    // (the actual rendering resolution, not the number of onscreen pixels.)

    let mut dim = get_viewport_dim(window);
    //alert(&format!("dim {:?}", dim ));
    adjust_aspect(&mut dim, aspect_ratio);
    //alert(&format!("dim {:?}", dim ));
    dim.0 *= super_sampling_ratio * screen_ratio; 
    dim.1 *= super_sampling_ratio * screen_ratio; 
    (dim.0 as u32, dim.1 as u32)
}

fn get_screen_res(
        window: &Window,         
        screen_ratio: f32, 
        aspect_ratio: (f32, f32)) -> (u32, u32) {
    // gets the dimensions of the image drawn to the screen

    let mut dim = get_viewport_dim(window);
    adjust_aspect(&mut dim, aspect_ratio);
    ((dim.0 * screen_ratio) as u32, (dim.1 * screen_ratio) as u32)
}

fn handle_resize(wp: Webpage) {
    let window = wp.window.clone();
    let window = window.borrow();
    let context = wp.context.clone();
    let context = context.borrow();
    let document = wp.document.clone();
    let document = document.borrow();
    let aspect_ratio = (GAME_ASPECT_X, GAME_ASPECT_Y);
    let screen_ratio = DEFAULT_SCREEN_RATIO;
    let super_sampling_ratio = DEFAULT_SUPER_SAMPLING_RATIO;
    let canvas = document.get_element_by_id("canvas").unwrap();
    let body: HtmlElement = canvas.parent_element().unwrap().dyn_into::<HtmlElement>().unwrap();
    let canvas:HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let (pixels_x, pixels_y) = get_screen_res(&*window, screen_ratio, aspect_ratio);
    let (raster_x, raster_y) = get_raster_res(
        &window, 
        super_sampling_ratio, 
        screen_ratio,
        aspect_ratio);

   // let (width, height) = get_viewport_dim(&*window);
    canvas.set_width(raster_x);
    canvas.set_height(raster_y);
    //alert(&format!("uni {:?}", (raster_x, raster_y)));
   if raster_x > raster_y  {
        canvas.style().set_property("height", &pixels_y.to_string()).unwrap();
        alert(&format!("height bound {:?} {:?}", (raster_x, raster_y), (pixels_x, pixels_y)));
    } else {
        canvas.style().set_property("width", &pixels_x.to_string()).unwrap();
        alert(&format!("width bound {:?}", (raster_x, raster_y)));
    }

   // MAKE RESIZABLE
   context.scissor(0, 0, pixels_x.try_into().unwrap(), pixels_y.try_into().unwrap());

    context.viewport(0, 0, raster_x.try_into().unwrap(), raster_y.try_into().unwrap());
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

    let aspect_ratio = (GAME_ASPECT_X, GAME_ASPECT_Y);
    let screen_ratio = DEFAULT_SCREEN_RATIO;
    let super_sampling_ratio = DEFAULT_SUPER_SAMPLING_RATIO;

    let (pixels_x, pixels_y) = get_screen_res(&*window, screen_ratio, aspect_ratio);
    let (raster_x, raster_y) = get_raster_res(
        &window, 
        super_sampling_ratio, 
        screen_ratio,
        aspect_ratio);

    body.style().set_property("margin", "0px")?;
    canvas.style().set_property("position", "relative")?;
    canvas.style().set_property("margin", "auto")?;

    alert(&format!("{:?} {:?}", &(pixels_x, pixels_y), & (raster_x, raster_y)));
    
    canvas.set_width(raster_x);
    canvas.set_height(raster_y);

    if raster_x > raster_y  {
        canvas.style().set_property("height", &pixels_y.to_string())?;
    } else {
        canvas.style().set_property("width", &pixels_x.to_string())?;
    }
    canvas.style().set_property("display", "block")?;
    canvas.style().set_property("aspect-ratio", &format!("{} / {}", aspect_ratio.0, aspect_ratio.1))?;

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
    let resize_wp = wp.clone();
    let resize_handler = Closure::<dyn FnMut()>::new(move || { 
            handle_resize(resize_wp.clone());
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
        aspect_ratio,
        screen_ratio,
        super_sampling_ratio,
        camera: Camera::default()
    };


    let mut game = Game::new();
    game.load_stage(&DemoStage {});

    start_game_loop(game_context, game);
    window.set_onresize(Some(resize_handler.as_ref().unchecked_ref()));
    resize_handler.forget();
    
    Ok(())
}