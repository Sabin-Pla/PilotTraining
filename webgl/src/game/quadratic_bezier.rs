use crate::*;

#[derive(Copy, Clone, Default)]
pub struct QuadraticBezier {
	start_node: FieldPosition,
	control_node: FieldPosition,
	end_node: FieldPosition
}

pub type BezierTuple = (FieldPosition, FieldPosition, FieldPosition);

impl QuadraticBezier {
	pub fn from(nodes: BezierTuple) -> Self {
		Self {
			start_node: nodes.0,
			control_node: nodes.1,
			end_node: nodes.2
		}
	}

	pub fn to_padded_buffer(&self, aspect_ratio: (f32, f32), field_offset: (f32, f32)) -> [f32; 12] {
		let start_node = self.start_node.clipspace(aspect_ratio, field_offset);
		let control_node = self.control_node.clipspace(aspect_ratio, field_offset);
		let end_node = self.end_node.clipspace(aspect_ratio, field_offset);
		[
			start_node.0, start_node.1, 
			control_node.0, control_node.1, 
			end_node.0, end_node.1, 
			0.0, 0.0, 0.0, 0.0, 0.0, 0.0
		]
	}
}