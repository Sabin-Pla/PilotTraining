mod text_renderer;

use crate::*;

pub const UNIFORM_CAMERA_IDX: usize = 1;
pub const UNIFORM_NODES_IDX: usize = 2;
pub const UNIFORM_RESOLUTION_IDX: usize = 3;
pub const UNIFORM_IMAGE_IDX: usize = 4;

pub use text_renderer::*;

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
	Attribute     {
		datatype: BufferDataType, 
		name: String,
		dim_len: usize },
	Uniform     {
		datatype: BufferDataType, 
		name: String,
		idx: usize },
	ElementArray,
	Vertexes      (usize),
	Texture       {  
		datatype: BufferDataType,
		width: u32, 
		height: u32,
		data: Vec<u8> } 
}

impl BufferArg {
	fn target(&self) -> BufferTarget {
		match self {
			Self::Attribute { .. } | Self::Vertexes { .. } | Self::Texture { .. } => BufferTarget::ArrayBuffer,
			Self::Uniform { .. } => BufferTarget::UniformBuffer,
			Self::ElementArray => BufferTarget::ElementArrayBuffer
		}
	}

	fn datatype(&self) -> BufferDataType {
		match self {
			Self::Attribute { datatype, .. } => *datatype,
			Self::Uniform { datatype,   .. } => *datatype,
			Self::ElementArray => UnsignedInt,
			Self::Vertexes(..) => Float,
			Self::Texture { datatype, .. } => *datatype
		}
	}
}

pub fn initialize_base_shaders(context: &WebGl2RenderingContext) -> [(WebGlShader, WebGlShader); 3] {
	let interface_vertex = compile_shader(
        &context, WebGl2RenderingContext::VERTEX_SHADER,
        &std::include_str!("../shaders/interface/vert.glsl")).unwrap();
    let interface_fragment = compile_shader(
        &context, WebGl2RenderingContext::FRAGMENT_SHADER,
        &std::include_str!("../shaders/interface/frag.glsl")).unwrap();
    let track_vertex = compile_shader(
        &context, WebGl2RenderingContext::VERTEX_SHADER,
        &std::include_str!("../shaders/track/vert.glsl")).unwrap();
    let track_fragment = compile_shader(
        &context, WebGl2RenderingContext::FRAGMENT_SHADER,
        &std::include_str!("../shaders/track/frag.glsl")).unwrap();
    let text_vertex = compile_shader(
        &context, WebGl2RenderingContext::VERTEX_SHADER,
        &std::include_str!("../shaders/text/vert.glsl")).unwrap();
    let text_fragment = compile_shader(
        &context, WebGl2RenderingContext::FRAGMENT_SHADER,
        &std::include_str!("../shaders/text/frag.glsl")).unwrap();
    [(interface_vertex, interface_fragment), 
     (track_vertex, track_fragment),
     (text_vertex, text_fragment)]
}

pub fn load_buffer<T: std::clone::Clone + std::fmt::Debug>(
		buf: &[T], arg: BufferArg,
		context: &WebGl2RenderingContext,
		program: &WebGlProgram) {

	// todo: validate if buffer param type is incorrect given bufferarg
	// this can easily happen in the even that we don't pass an explicit type and rust just defaults to f64

	let gl_buffer = context.create_buffer().expect("Failed to create buffer");
	let buffer_target = arg.target();
    context.bind_buffer(buffer_target.websys_code(), Some(&gl_buffer));

    let bind_uniform = |name: &str, idx: u32| { 
    	let uniform_index = context.get_uniform_block_index(&program, &name);
		context.uniform_block_binding(
	        &program, uniform_index, idx);
		context.bind_buffer_base(
	        WebGl2RenderingContext::UNIFORM_BUFFER, idx, Some(&gl_buffer));
    };

	match arg {
		BufferArg::Attribute { datatype,  ref name, dim_len} => 
			set_attribute(context, program, datatype, name, dim_len),

		BufferArg::Uniform { datatype, ref name, idx} => bind_uniform(name, idx  as u32),

		BufferArg::Vertexes(dim_len) => {
			create_vertex_buffer(context);
			set_attribute(context, program, arg.datatype(), "position", dim_len);
		},

		BufferArg::Texture{ datatype, width, height, ref data } => {
			bind_texture_buffers(buf, context, program, datatype, width, height, data);
			return;
		},

		BufferArg::ElementArray => {}, // nothing to do
	}

    unsafe {
        let buffer_js = js_array!(arg.datatype(), buf);
        context.buffer_data_with_array_buffer_view(
        	buffer_target.websys_code(), &buffer_js, WebGl2RenderingContext::STATIC_DRAW);
    }
}

fn set_attribute(
	context: &WebGl2RenderingContext, 
	program: &WebGlProgram,
	datatype: BufferDataType, 
	name: &str, dim_len: usize) {

	let attribute_location = context.get_attrib_location(&program, &name) as u32;
    context.vertex_attrib_pointer_with_i32(
        attribute_location, 
        dim_len.try_into().unwrap(), 
        datatype.websys_code(),  false, 0, 0);
    context.enable_vertex_attrib_array(attribute_location);
}

fn create_vertex_buffer(context: &WebGl2RenderingContext) {
	let vao = context
	        .create_vertex_array()
	        .expect("Could not create vertex array object");
	context.bind_vertex_array(Some(&vao));
}

fn bind_texture_buffers<T: std::clone::Clone + std::fmt::Debug>(
		buf: &[T], 
		context: &WebGl2RenderingContext,
		program: &WebGlProgram,
		datatype: BufferDataType,
		width: u32, 
		height: u32,
		texture_data: &Vec<u8>
	) {

	unsafe {
		let buffer_js = js_array!(BufferDataType::Float, buf);
    	context.buffer_data_with_array_buffer_view(
    		WebGl2RenderingContext::ARRAY_BUFFER, &buffer_js, WebGl2RenderingContext::STATIC_DRAW);
	}
	create_vertex_buffer(context);
	set_attribute(context, program, BufferDataType::Float, "a_position", 2);

	let texture_unit_number = 0;
	let tex_buffer = context.create_buffer().expect("Failed to create tex buf");

    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&tex_buffer));
	unsafe {
		let new_data: &[f32] = &[
			0.0_f32, 0.0, 
			0.0, 1.0, 
			1.0, 1.0,
			1.0, 0.0, 
			];
		let new_data: &[T] = std::mem::transmute::<&[f32], &[T]>(new_data);

		let buffer_js = js_array!(BufferDataType::Float, new_data);
    	context.buffer_data_with_array_buffer_view(
    		WebGl2RenderingContext::ARRAY_BUFFER, &buffer_js, WebGl2RenderingContext::STATIC_DRAW);
	}
	set_attribute(context, program, BufferDataType::Float, "a_texCoord", 2);

	let texture = context.create_texture().expect("failed to create webgl texture");
	let texture_uniform_location = context.get_uniform_location(program, "u_image")
		.expect("could not find u_image uniform");
	context.active_texture(WebGl2RenderingContext::TEXTURE0 + texture_unit_number);
	context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
	
	set_tex_param(context);		
	context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
		WebGl2RenderingContext::TEXTURE_2D.try_into().unwrap(),
		0,
		WebGl2RenderingContext::R8UI.try_into().unwrap(),
		width.try_into().unwrap(),
		height.try_into().unwrap(),
		0,
		WebGl2RenderingContext::RED_INTEGER.try_into().unwrap(),
		WebGl2RenderingContext::UNSIGNED_BYTE.try_into().unwrap(),
		&texture_data,
		0,
	);

	context.uniform1i(Some(&texture_uniform_location), texture_unit_number as i32);
}

fn set_tex_param(context: &WebGl2RenderingContext) {
	// sets the defualt text params to disable wrap/mipmap levels
	
	context.tex_parameteri(
		WebGl2RenderingContext::TEXTURE_2D, 
		WebGl2RenderingContext::TEXTURE_WRAP_S, 
		WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
	context.tex_parameteri(
		WebGl2RenderingContext::TEXTURE_2D, 
		WebGl2RenderingContext::TEXTURE_WRAP_T, 
		WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
	context.tex_parameteri(
		WebGl2RenderingContext::TEXTURE_2D, 
		WebGl2RenderingContext::TEXTURE_MIN_FILTER,
		WebGl2RenderingContext::NEAREST as i32 );
	context.tex_parameteri(
		WebGl2RenderingContext::TEXTURE_2D, 
		WebGl2RenderingContext::TEXTURE_MAG_FILTER,
		WebGl2RenderingContext::NEAREST as i32);	
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

