use crate::*;

pub trait Stage {
	fn track_spline(&self) -> Vec<FieldPosition>;
}

const DEMO_STAGE_BEZIER_NODES: [f32; 6] = [
	-0.5, -0.5,
	-0.0, 0.5,
	0.5, -0.5];


pub struct DemoStage {}

impl Stage for DemoStage {
	fn track_spline(&self) -> Vec<FieldPosition> {
		let offset = -0.4;
		DEMO_STAGE_BEZIER_NODES
			.chunks_exact(2).map(|v| 
				FieldPosition {
					x: v[0] / 2.0 , 
					y: v[1] 
				})
			.collect()
	}
}