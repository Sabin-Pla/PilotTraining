use crate::*;

pub trait Stage {
	fn track_spline(&self) -> Vec<QuadraticBezier>;
}

const DEMO_STAGE_BEZIER_NODES: [f32; 6] = [
	-0.8, -0.2,
	-0.2, 0.5,
	0.9, -0.39];


pub struct DemoStage {}

impl Stage for DemoStage {
	fn track_spline(&self) -> Vec<QuadraticBezier> {
		DEMO_STAGE_BEZIER_NODES
			.windows(2).map(|v| 
				FieldPosition {
					x: v[0], 
					y: v[1] 
				})
			.collect::<Vec<_>>()
			.windows(3).map(
				|t| QuadraticBezier::from(
					(t[0], t[1], t[2])))
			.collect()
	}
}