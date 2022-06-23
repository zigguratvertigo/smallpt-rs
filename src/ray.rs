use bvh::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
	pub origin: Vector3,
	pub direction: Vector3,
}

impl Ray {
	pub fn new(origin: Vector3, direction: Vector3) -> Ray {
		Ray { origin, direction }
	}
}
