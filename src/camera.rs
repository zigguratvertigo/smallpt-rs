use bvh::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
	pub origin: Vector3,
	pub forward: Vector3,
	pub right: Vector3,
	pub up: Vector3,
}

impl Camera {
	pub fn new(origin: Vector3, forward: Vector3, right: Vector3, up: Vector3) -> Camera {
		Camera {
			origin,
			forward,
			right,
			up,
		}
	}
}
