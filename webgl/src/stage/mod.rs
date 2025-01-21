use crate::*;

pub trait Stage {
	fn track_spline(&self) -> Vec<FieldPosition>;
}

const DEMO_STAGE_BEZIER_NODES: [f32; 6] = [
	-1.5, -0.9,
	-0.0, 0.9,
	3.0, -0.5];


pub struct DemoStage {}

impl Stage for DemoStage {
	fn track_spline(&self) -> Vec<FieldPosition> {
		DEMO_STAGE_BEZIER_NODES
			.chunks_exact(2).map(|v| 
				FieldPosition {
					x: v[0], 
					y: v[1] 
				})
			.collect()
	}
}