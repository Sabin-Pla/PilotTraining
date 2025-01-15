use crate::*;

pub const UNIFORM_CAMERA_IDX: usize = 1;
pub const UNIFORM_NODES_IDX: usize = 2;
pub const UNIFORM_RESOLUTION_IDX: usize = 3;

macro_rules! js_array {
	( $attribute_type:tt, $buf_type:ty, $buf: ident ) => {
		{
	    	let js_array = js_sys::$attribute_type::new_with_length($buf.len() as u32);
	    	let buf: &[$buf_type] = std::mem::transmute::<&[T], &[$buf_type]>($buf);
	    	js_array.copy_from(buf);
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

pub enum BufferArg {
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

pub fn initialize_base_shaders(context: &WebGl2RenderingContext) -> [(WebGlShader, WebGlShader); 2] {
	let interface_vertex = compile_shader(
        &context, WebGl2RenderingContext::VERTEX_SHADER,
        &std::include_str!("shaders/interface/vert.glsl")).unwrap();
    let interface_fragment = compile_shader(
        &context, WebGl2RenderingContext::FRAGMENT_SHADER,
        &std::include_str!("shaders/interface/frag.glsl")).unwrap();
    let track_vertex = compile_shader(
        &context, WebGl2RenderingContext::VERTEX_SHADER,
        &std::include_str!("shaders/track/vert.glsl")).unwrap();
    let track_fragment = compile_shader(
        &context, WebGl2RenderingContext::FRAGMENT_SHADER,
        &std::include_str!("shaders/track/frag.glsl")).unwrap();
    [(interface_vertex, interface_fragment), (track_vertex, track_fragment)]
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
    frag_shader: &WebGlShader
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