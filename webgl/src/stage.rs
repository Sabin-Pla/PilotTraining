use crate::*;

const PLATFORM_VERT: [f32; 8] = [
    -1.0, -1.0, 
    -1.0, 0.0, 
    1.0, -1.0,
    1.0, 0.0,
];

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

pub fn init(context: &WebGl2RenderingContext, program: &WebGlProgram) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    let index_buffer = [0, 1, 2, 3];
    load_buffer(&PLATFORM_VERT, BufferArg::Vertexes(2), context, program);
    load_buffer(&index_buffer, BufferArg::ElementArray, context, program);
    context.draw_elements_with_i32(
        WebGl2RenderingContext::TRIANGLE_STRIP, index_buffer.len() as i32, 
        WebGl2RenderingContext::UNSIGNED_INT, 0);
}


enum BufferArg {
	Attribute(BufferDataType, usize, String),
	Uniform  (BufferDataType, String, usize),
	ElementArray,
	Vertexes (usize)
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

	let gl_buffer = context.create_buffer().expect("Failed to create buffer");
	let buffer_target = arg.target();
    context.bind_buffer(buffer_target.websys_code(), Some(&gl_buffer));

    let set_attribute = |datatype: BufferDataType, name: &str, dim_len: usize| {
    	let attribute_location = context.get_attrib_location(&program, &name);
	    context.vertex_attrib_pointer_with_i32(
	        attribute_location as u32, dim_len.try_into().unwrap(), datatype.websys_code(), false, 0, 0);
	    context.enable_vertex_attrib_array(attribute_location as u32);
    };

	match arg {
		BufferArg::Attribute(datatype, dim_len, ref name) => set_attribute(datatype, name, dim_len),

		BufferArg::Uniform(datatype, ref name, idx) => {
			context.uniform_block_binding(
		        &program, context.get_uniform_block_index(&program, &name), idx as u32);
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
