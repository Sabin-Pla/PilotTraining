use crate::*;

pub use BufferDataType::*;

// https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer#type
#[derive(Copy, Clone)]
pub enum BufferDataType {
	Byte,
	Short,
	UnsignedByte,
	UnsignedShort,
	Float, // 32 bit
	HalfFloat,
	Int,
	UnsignedInt,
}

#[derive(Copy, Clone)]
pub enum BufferTarget {
	ArrayBuffer,
	ElementArrayBuffer,
	CopyReadBuffer,
	CopyWriteBuffer,
	TransformFeedbackBuffer,
	UniformBuffer,
	PixelPackBuffer,
	PixelUnpackBuffer
}

impl BufferTarget {
	pub fn websys_code(&self) -> u32 {
		match self {
			Self::ArrayBuffer => WebGl2RenderingContext::ARRAY_BUFFER,
			Self::ElementArrayBuffer => WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
			Self::CopyReadBuffer => WebGl2RenderingContext::COPY_READ_BUFFER,
			Self::CopyWriteBuffer => WebGl2RenderingContext::COPY_WRITE_BUFFER,
			Self::TransformFeedbackBuffer => WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER,
			Self::UniformBuffer => WebGl2RenderingContext::UNIFORM_BUFFER,
			Self::PixelPackBuffer => WebGl2RenderingContext::PIXEL_PACK_BUFFER,
			Self::PixelUnpackBuffer => WebGl2RenderingContext::PIXEL_UNPACK_BUFFER
		}
	}
}

impl BufferDataType {
	pub fn websys_code(&self) -> u32 {
		match self {
			Self::Byte => WebGl2RenderingContext::BYTE,
			Self::Short => WebGl2RenderingContext::SHORT,
			Self::UnsignedByte => WebGl2RenderingContext::UNSIGNED_BYTE,
			Self::UnsignedShort => WebGl2RenderingContext::UNSIGNED_SHORT,
			Self::Float => WebGl2RenderingContext::FLOAT, // 32 bit
			Self::HalfFloat => WebGl2RenderingContext::HALF_FLOAT,
			Self::Int => WebGl2RenderingContext::INT,
			Self::UnsignedInt => WebGl2RenderingContext::UNSIGNED_INT,
		}
	}
}