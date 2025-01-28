use crate::*;

use fontdue::*;
use fontdue::layout::*;

pub fn draw_text(context: &mut GameContext) {
	let wgl_context = context.wp.context.clone();
	let wgl_context = wgl_context.borrow();
	let program = &context.text_shader;
	wgl_context.use_program(Some(program));

	let font = include_bytes!("../resources/fonts/dimica/Dimica-Light.otf") as &[u8];
	let dimica_light = Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
	let fonts = &[dimica_light];
	// Create a layout context. Laying out text needs some heap allocations; reusing this context
	// reduces the need to reallocate space. We inform layout of which way the Y axis points here.
	let mut layout = Layout::new(CoordinateSystem::PositiveYUp);
	// By default, layout is initialized with the default layout settings. This call is redundant, but
	// demonstrates setting the value with your custom settings.
	layout.reset(&LayoutSettings {
	    ..LayoutSettings::default()
	});
	// The text that will be laid out, its size, and the index of the font in the font list to use for
	// that section of text.
	layout.append(fonts, &TextStyle::new("Hello ", 35.0, 0));
	layout.append(fonts, &TextStyle::new("world!", 40.0, 0));
	// Prints the layout for "Hello world!"
	println!("{:?}", layout.glyphs());

	let g = layout.glyphs()[0];
	let key = g.key;
	let (metrics, bitmap) = fonts[0].rasterize_indexed(key.glyph_index, key.px);

	// take up bottom left of screen
	let bounding_box: [f32; 8] = [
		-1.0, 1.0,
		-1.0, 0.0,
		0.0, 0.0,
        0.0, 1.0];

    let buffer_arg = BufferArg::Texture {
    	datatype: BufferDataType::UnsignedByte,
    	width: metrics.width as u32,
		height: metrics.height as u32,
		data: bitmap
    };
	load_buffer(&bounding_box, buffer_arg, &wgl_context, program);
	wgl_context.draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, (bounding_box.len() / 2) as i32);
}

