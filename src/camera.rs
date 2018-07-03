use nalgebra::*;
type Vec3 = Vector3<f32>;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
	pub origin: Vec3,
	pub forward: Vec3,
	pub right: Vec3,
	pub up: Vec3,
}

impl Camera {
	pub fn new(origin: Vec3, forward: Vec3, right: Vec3, up: Vec3) -> Camera {
		Camera {
			origin: origin,
			forward: forward,
			right: right,
			up: up,
		}
	}
}
