use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use web_sys::*;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}


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

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("Failed to start WASM: window()");
    let window_cell = Rc::new(RefCell::new(window));
    let window = window_cell.clone();
    let window = window.borrow_mut();
    let document = window.document().unwrap();

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
        in vec4 indexes;
        out vec4 index2;
        out vec4 pos2;

        layout (std140) uniform CircleCenters {
            vec4 u_circleCenters[2];
        };
        out vec2 center; // Circle index

        void main() {
            gl_Position = position;
            pos2 = position;
            int i = int(indexes[0]);
            center = u_circleCenters[i].xy;     
            index2 = indexes;
        }
        "##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
    
        precision highp float;
        in vec4 position;
        in vec4 index2;
        in vec4 pos2;
        out vec4 outColor;
        in vec2 center; 
        
        void main() {
            float dist = distance(pos2.xy, center.xy);
            if (dist > 0.1) { 
                float c = 0.2;

                 outColor = vec4(index2.x, index2.y, 0.1, 1);
            } else {
                float c = 0.35;
                 outColor = vec4(c, c, c, 1);
            }
        }
        "##,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices = [
        -1.0, -1.0, 
        0.0, -0.3, 
        -0.3, 0.3,

         -0.2, -1.0, 
        0.1, -0.21, 
        0.7, 0.0,
    ];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
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
        2, WebGl2RenderingContext::FLOAT, false, 0, 0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);


    let index_buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    let mut indexes = [0 as u32, 1, 2, 3, 4, 5];
    unsafe {
        let indexes_js = js_sys::Uint32Array::new_with_length(6);
        indexes_js.copy_from(&indexes);
        // alert(&format!("Hello, {:?}!", &indexes_js.to_string()));
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &indexes_js, WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let index_buffer2 = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&index_buffer2));
    let mut indexes2 = [0 as u32, 0, 0, 1, 1, 1];
    unsafe {
        let indexes_js = js_sys::Uint32Array::new_with_length(6);
        indexes_js.copy_from(&indexes2);
        // alert(&format!("Hello, {:?}!", &indexes_js.to_string()));
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &indexes_js, WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let indexes_attribute_location = context.get_attrib_location(&program, "indexes");
   context.vertex_attrib_pointer_with_i32(
        indexes_attribute_location as u32,
        1, WebGl2RenderingContext::UNSIGNED_INT, false, 0, 0,
    );
    context.enable_vertex_attrib_array(indexes_attribute_location as u32);




    let mut centroids = vec!();
    for triangle_verts in vertices.chunks_exact(6) {
        let (mut x, mut y) = (0.0, 0.0);
        for point in triangle_verts.chunks_exact(2) {
            x += point[0];
            y += point[1];
        }

        x /= 3.0;
        y /= 3.0;
        centroids.push(x);
        centroids.push(y);
        // we need to append 2 extra elements because the buffer
        // must be rounded up to the base alignment of a vec4
        // https://registry.khronos.org/OpenGL/specs/es/3.0/es_spec_3.0.pdf 
        // Section 2.12
        centroids.push(0.0); 
        centroids.push(0.0);
    }
    let centroid_buffer_idx = 1;
    context.uniform_block_binding(
        &program, 
        context.get_uniform_block_index(&program, "CircleCenters"),
        centroid_buffer_idx);
    let centroid_buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, Some(&centroid_buffer));
    unsafe {
        let centroids_js = js_sys::Float32Array::new_with_length(centroids.len() as u32);
        centroids_js.copy_from(&centroids.as_slice());
        // alert(&format!("Hello, {:?}!", &centroids_js.to_string()));
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::UNIFORM_BUFFER,
            &centroids_js,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
    context.bind_buffer_base(
        WebGl2RenderingContext::UNIFORM_BUFFER,
        centroid_buffer_idx,
        Some(&centroid_buffer));


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

    //context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
    context.draw_elements_with_i32(
        WebGl2RenderingContext::TRIANGLES, 
        6, 
        WebGl2RenderingContext::UNSIGNED_INT,
        0);
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