use bvh::Vector3;
use material::Material;
use std::f32::INFINITY;

#[derive(Copy, Clone)]
pub struct Hit {
	pub p: Vector3,
	pub n: Vector3,
	pub t: f32,
	pub b: Vector3,
	pub material: Material,
}

impl Hit {
	// Spawn a new Hit result data structure
	pub fn new(p: Vector3, n: Vector3, t: f32, b: Vector3, material: Material) -> Hit {
		Hit {
			p,
			n,
			t,
			b,
			material,
		}
	}

	pub fn init() -> Hit {
		Hit {
			p: Vector3::new(0.0, 0.0, 0.0),
			n: Vector3::new(0.0, 0.0, 0.0),
			t: INFINITY,
			b: Vector3::new(0.0, 0.0, 0.0),
			material: Material::black(),
		}
	}
}
