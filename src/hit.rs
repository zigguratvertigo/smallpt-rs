use crate::material::Material;
use crate::Vec3;
use std::f32::INFINITY;

#[derive(Copy, Clone)]
pub struct Hit {
	pub p: Vec3,
	pub n: Vec3,
	pub t: f32,
	pub b: Vec3,
	pub material: Material,
}

impl Hit {
	pub fn new(p: Vec3, n: Vec3, t: f32, b: Vec3, material: Material) -> Hit {
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
			p: Vec3::new(0.0, 0.0, 0.0),
			n: Vec3::new(0.0, 0.0, 0.0),
			t: INFINITY,
			b: Vec3::new(0.0, 0.0, 0.0),
			material: Material::black(),
		}
	}
}
