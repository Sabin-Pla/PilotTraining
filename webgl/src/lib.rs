use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use web_sys::*;
use std::cell::RefCell;
use std::rc::Rc;

fn get_viewport_dim(window: &Window) -> (u32, u32) {
    let width = window.outer_width().unwrap().as_f64().unwrap() * 0.8;
    let height = window.outer_height().unwrap().as_f64().unwrap() * 0.8;
    (width as u32, height as u32)
}

fn handle_resize( 
        canvas: Rc<RefCell<HtmlCanvasElement>>,
        context: Rc<RefCell<WebGl2RenderingContext>>,
        window: Rc<RefCell<Window>>,
        vert_count: i32) {
    let window = window.borrow_mut();
    let canvas = canvas.borrow_mut();
    let context = &context.borrow_mut();
    let (width, height) = get_viewport_dim(&*window);
    canvas.set_width(width);
    canvas.set_height(height);
    context.viewport(0, 0,  
        canvas.width().try_into().unwrap(),
        canvas.height().try_into().unwrap());
    draw(context, vert_count);

}


// https://rustwasm.github.io/docs/wasm-bindgen/examples/paint.html

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("Failed to start WASM: window()");
    let window_cell = Rc::new(RefCell::new(window));
    let window = window_cell.clone();
    let window = window.borrow_mut();
    let document = window.document().unwrap();

   // let body = document.get_element_by_id("body").unwrap();
    //let mut body: HtmlElement = body.dyn_into::<web_sys::HtmlElement>()?;

    //println!("{:?}", &body);
    let canvas = document.get_element_by_id("canvas").unwrap();
    let body: HtmlElement = canvas.parent_element().unwrap().dyn_into::<HtmlElement>()?;
    let mut canvas:HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let (width, height) = get_viewport_dim(&*window);
    canvas.set_width(width);
    canvas.set_height(height);

    body.style().set_property("width", "100%")?;
    body.style().set_property("margin", "0px")?;
    canvas.style().set_property("position", "relative")?;
    canvas.style().set_property("margin", "auto")?;
    canvas.style().set_property("max-width", "80%")?;
    canvas.style().set_property("height", "100%")?;
    canvas.style().set_property("display", "block")?;
    canvas.style().set_property("aspect-ratio", "1 / 1")?;

    let canvas_cell = Rc::new(RefCell::new(canvas));
    let canvas = canvas_cell.clone();
    let canvas = canvas.borrow_mut();
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;
    let context_cell = Rc::new(RefCell::new(context));
    let context = context_cell.clone();
    let context = &context.borrow_mut();



    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
 
        in vec4 position;

        void main() {
        
            gl_Position = position;
        }
        "##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
    
        precision highp float;
        out vec4 outColor;
        
        void main() {
            outColor = vec4(1, 1, 1, 1);
        }
        "##,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 18] = [
        -1.0, -1.0, 0.0, 
        1.0, -0.21, 0.0, 
        0.0, 0.7, 0.0,

         -0.2, -1.0, 0.0, 
        0.1, -0.21, 0.0, 
        0.7, 0.0, 0.0,

        ];


    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));


    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));



    let vert_count = (vertices.len() / 3) as i32;
    draw(&(*context), vert_count);

    let resize_handler = Closure::<dyn FnMut()>::new(move || { 
        handle_resize(
            canvas_cell.clone(),
            context_cell.clone(),
            window_cell.clone(),
            vert_count)
        }
    );
    //window.set_onresize(Some(resize_handler.as_ref().unchecked_ref()));
    resize_handler.forget();
    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false) {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}