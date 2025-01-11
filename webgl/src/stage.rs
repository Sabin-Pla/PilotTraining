use crate::*;


type Rectangle = [f32; 8];

const PLATFORM_VERT: Rectangle = [
    -1.0, -0.5, 
    -1.0, 0.5, 
    1.0, -0.5,
    1.0, 0.5,
];

const PLAYER_VERT: Rectangle = [
	-0.2, -0.2,
	-0.2, 0.2,
	0.2, -0.2,
	0.2, 0.2
];

const OBJECT_OFFSETS: [[f32; 2]; 2] = [	
	[0.3, -0.9],  // player
	[0.0, 0.0]  // platform
];

const OBJECT_COLORS: [[f32; 4]; 2] = [
	[1.0, 1.0, 0.2, 0.9], // platform
	[0.2, 1.0, 1.0, 0.4]  // player
]; 

const UNIFORM_COLORS_IDX: usize = 4;
const UNIFORM_CENTERS_IDX: usize = 3;
const UNIFORM_CAMERA_IDX: usize = 2;
const UNIFORM_NODES_IDX: usize = 5;
const UNIFORM_RESOLUTION_IDX: usize = 6;

struct World<'a> {
	positions: Vec<(f64, f64)>,
	objects: Vec<(&'a [f64], ObjectDescription)>,
	names: HashMap<String, usize>
}

struct ObjectDescription {
	colour: (u8, u8, u8),
}

macro_rules! js_array {
	( $attribute_type:tt, $buf_type:ty, $buf: ident ) => {
		{
	    	let js_array = js_sys::$attribute_type::new_with_length($buf.len() as u32);
	    	let buf: &[$buf_type] = std::mem::transmute::<&[T], &[$buf_type]>($buf);
	    	js_array.copy_from(buf);
	    	// alert(&format!("Hello! {}", js_array.to_string()));
	    	js_array.value_of()
	    }
    };

    ( $datatype:expr, $buf:ident ) => {
        {
	        match $datatype {
	        	UnsignedInt => js_array!(Uint32Array, u32, $buf),
				Byte =>  js_array!(Int8Array, i8, $buf),
				Short =>  js_array!(Int16Array, i16, $buf),
				UnsignedByte =>  js_array!(Uint8Array, u8, $buf),
				UnsignedShort =>  js_array!(Uint16Array, u16, $buf),
				Float =>  js_array!(Float32Array, f32, $buf),
				HalfFloat => panic!("No corresponding js_sys array type for HalfFloat"),
				Int =>  js_array!(Int32Array, i32, $buf),
	        }
        }
    };
}


fn rectangle_center(r: Rectangle, offset: usize) -> [f32; 4] {
	let offset = OBJECT_OFFSETS[offset];
	[(r[0] + r[2] + r[4] + r[6]) / 4.0, (r[1] + r[3] + r[5] + r[7]) / 4.0, offset[0], offset[1]]
}

pub fn stage_program(context: &WebGl2RenderingContext) ->  Result<WebGlProgram, String> {

	let shader_err = |t| { 
		let inner = move |e| {
			format!("{t} shader error\n{e}") 
		};
		inner
	};

	 let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        &std::include_str!("stage_vertex.glsl")).map_err(shader_err("Vertex"));

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        &std::include_str!("stage_fragment.glsl")).map_err(shader_err("Fragment"));

    link_program(&context, &vert_shader?, &frag_shader?)
}

pub fn stage_program_bezier(context: &WebGl2RenderingContext) ->  Result<WebGlProgram, String> {

	let shader_err = |t| { 
		let inner = move |e| {
			format!("{t} shader error\n{e}") 
		};
		inner
	};

	 let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        &std::include_str!("bezier_vertex.glsl")).map_err(shader_err("Vertex"));

    let bezier_frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        &std::include_str!("bezier_fragment.glsl")).map_err(shader_err("Fragment"));

    link_program(&context, &vert_shader?, &bezier_frag_shader?)
}

pub fn init(context: &WebGl2RenderingContext, program: &WebGlProgram, window: &Window) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let camera = Camera { center:(0.0, -0.0), zoom: (0.125, 0.125)};

    let objects = [&PLAYER_VERT, &PLATFORM_VERT];
    let object_verts = objects.iter().map(|o| **o).into_iter().flatten().collect::<Vec<f32>>();
    let centers = objects.iter().enumerate().map(|(offset, o)| rectangle_center(**o, offset))
    	.into_iter().flatten().collect::<Vec<f32>>();
    let index_buffer: Vec<usize> = (0..object_verts.len() / 2).collect();


    load_buffer(&object_verts, BufferArg::Vertexes(2), context, program);
    load_buffer(&index_buffer, BufferArg::ElementArray, context, program);
    load_buffer(
    	&index_buffer, 
    	BufferArg::Attribute(BufferDataType::UnsignedInt, 1, "vert_idx".to_string()), 
    	context, program);
    load_buffer(
    	&OBJECT_COLORS.into_iter().flatten().collect::<Vec<f32>>(), 
    	BufferArg::Uniform(BufferDataType::Float, "u_colors".to_string(), UNIFORM_COLORS_IDX), 
    	context, program);
    load_buffer(
    	&centers, 
    	BufferArg::Uniform(BufferDataType::Float, "u_worldspace_centers".to_string(), UNIFORM_CENTERS_IDX), 
    	context, program);
    load_buffer(
    	&camera.to_buffer(), 
    	BufferArg::Uniform(BufferDataType::Float, "u_camera".to_string(), UNIFORM_CAMERA_IDX), 
    	context, program);
    context.draw_arrays(
        WebGl2RenderingContext::TRIANGLE_STRIP, 0, 4);
    context.draw_arrays(
        WebGl2RenderingContext::TRIANGLE_STRIP, 4, 4);

   // game_loop(context, program, window);
}

pub fn init_bezier(context: &WebGl2RenderingContext, program: &WebGlProgram, window: &Window) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let camera = Camera { center:(0.0, -0.0), zoom: (0.125, 0.125)};

    let objects = [
    	-1.0, -0.2,
		-0.2, 0.5,
		0.9, -0.39, 0.0, 0.0];
    let index_buffer: Vec<usize> = vec![0, 1, 2, 3];


    load_buffer(&objects, BufferArg::Vertexes(2), context, program);
    load_buffer(&index_buffer, BufferArg::ElementArray, context, program);
    load_buffer(
    	&index_buffer, 
    	BufferArg::Attribute(BufferDataType::UnsignedInt, 1, "vert_idx".to_string()), 
    	context, program);
    let (width, height): (f32, f32) = (
    	window.inner_width().unwrap().as_f64().unwrap() as f32, 
    	window.inner_height().unwrap().as_f64().unwrap() as f32);
    let document = window.document().unwrap();
    let document_cell = Rc::new(RefCell::new(document));
    let document = document_cell.clone();
    let document = document.borrow_mut();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let body: HtmlElement = canvas.parent_element().unwrap().dyn_into::<HtmlElement>().unwrap();
    let mut canvas:HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let (width, height): (f32, f32) = (
    	canvas.width() as f32,
    	canvas.height() as f32);
    load_buffer(
    	&[width, height, 0.0, 0.0], 
    	BufferArg::Uniform(BufferDataType::Float, "u_resolution".to_string(), UNIFORM_RESOLUTION_IDX), 
    	context, program);
    load_buffer(
    	&camera.to_buffer(), 
    	BufferArg::Uniform(BufferDataType::Float, "u_camera".to_string(), UNIFORM_CAMERA_IDX), 
    	context, program);
    let nodes: [f32; 8] = objects.clone();
    load_buffer(
    	&nodes, 
    	BufferArg::Uniform(BufferDataType::Float, "u_nodes".to_string(), UNIFORM_NODES_IDX), 
    	context, program);
    context.draw_arrays(
        WebGl2RenderingContext::TRIANGLE_STRIP, 0, 3);
}

fn game_loop(context: &WebGl2RenderingContext, program: &WebGlProgram, window: &Window) {
	// https://github.com/anlumo/webgl_rust_demo/blob/master/src/renderer.rs
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let outer_f = f.clone();
    let context = context.clone();
    let window = window.clone();

    *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    	context.clear_color(0.0, 0.0, 0.0, 1.0);
   	 	context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting animation frame");
    }) as Box<dyn FnMut()>));

    let window = web_sys::window().unwrap();
    window.request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("failed requesting animation frame");
}

enum BufferArg {
	Attribute     (BufferDataType, usize, String),
	Uniform       (BufferDataType, String, usize),
	ElementArray,
	Vertexes      (usize)
}

impl BufferArg {
	fn target(&self) -> BufferTarget {
		match self {
			Self::Attribute(..)|Self::Vertexes(..) => BufferTarget::ArrayBuffer,
			Self::Uniform(..) => BufferTarget::UniformBuffer,
			Self::ElementArray => BufferTarget::ElementArrayBuffer
		}
	}

	fn datatype(&self) -> BufferDataType {
		match self {
			Self::Attribute(datatype, ..) => *datatype,
			Self::Uniform(datatype,   ..) => *datatype,
			Self::ElementArray => UnsignedInt,
			Self::Vertexes(..) => Float
		}
	}
}

pub fn load_buffer<T>(
		buf: &[T], arg: BufferArg,
		context: &WebGl2RenderingContext,
		program: &WebGlProgram) {

	// todo: validate if buffer param type is incorrect given bufferarg
	// this can easily happen in the even that we don't pass an explicit type and rust just defaults to f64

	let gl_buffer = context.create_buffer().expect("Failed to create buffer");
	let buffer_target = arg.target();
    context.bind_buffer(buffer_target.websys_code(), Some(&gl_buffer));

    let set_attribute = |datatype: BufferDataType, name: &str, dim_len: usize| {
    	let attribute_location = context.get_attrib_location(&program, &name);
	    context.vertex_attrib_pointer_with_i32(
	        attribute_location as u32, 
	        dim_len.try_into().unwrap(), 
	        datatype.websys_code(),  false, 0, 0);
	    context.enable_vertex_attrib_array(attribute_location as u32);
    };

	match arg {
		BufferArg::Attribute(datatype, dim_len, ref name) => set_attribute(datatype, name, dim_len),

		BufferArg::Uniform(datatype, ref name, idx) => {
			let uniform_index = context.get_uniform_block_index(&program, &name);
			context.uniform_block_binding(
		        &program, uniform_index, idx as u32);
			context.bind_buffer_base(
		        WebGl2RenderingContext::UNIFORM_BUFFER, idx as u32, Some(&gl_buffer));
		},

		BufferArg::Vertexes(dim_len) => {
			let vao = context
		        .create_vertex_array()
		        .expect("Could not create vertex array object");
		    context.bind_vertex_array(Some(&vao));
			set_attribute(arg.datatype(), "position", dim_len);
		},

		BufferArg::ElementArray => {}, // nothing to do
	}

    unsafe {
        let buffer_js = js_array!(arg.datatype(), buf);
        context.buffer_data_with_array_buffer_view(
        	buffer_target.websys_code(), &buffer_js, WebGl2RenderingContext::STATIC_DRAW);
    }
 }
