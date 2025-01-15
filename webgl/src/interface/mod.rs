use crate::*;

pub fn mouse_pos_bar_clipspace_vert(x_ratio: f32, y_ratio: f32) -> (u32, [f32; 16]) {
	// gets vertices of bars representing mouse x, y coords in clipspace.
	let widget_topleft = [0.9, -0.9];
	let widget_bottomright = [1.0, -1.0];

	let x0 = widget_topleft[0];
	let y0 = widget_topleft[1];
	let x1 = widget_bottomright[0];
	let y1 = widget_bottomright[1];
	let verts = [
		x0, y0, 
		x0 + (x1 - x0) * x_ratio, y0,
		x0, (y0 + y1) / 2.0,
		x0 + (x1 - x0) * x_ratio, (y0 + y1) / 2.0,

		x0, (y0 + y1) / 2.0,
		x0 + (x1 - x0) * y_ratio, (y0 + y1) / 2.0,
		x0, y1,
		x0 + (x1 - x0) * y_ratio, y1,
	];

	(WebGl2RenderingContext::TRIANGLE_STRIP, verts) 
}