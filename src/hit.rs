use material::Material;
use std::f32::*;
use nalgebra::*;
type Vec3 = Vector3<f32>;

#[derive(Copy, Clone)]
pub struct Hit {
	pub p: Vec3,
	pub n: Vec3,
	pub t: f32,
	pub b: Vec3,
	pub material: Material,
}

impl Hit {
	// Spawn a new Hit result data structure
	pub fn new(p: Vec3, n: Vec3, t: f32, b: Vec3, material: Material) -> Hit {
		Hit {
			p: p,
			n: n,
			t: t,
			b: b,
			material: material,
		}
	}

	pub fn init() -> Hit {
		Hit {
			p: Vec3::zeros(),
			n: Vec3::zeros(),
			t: INFINITY,
			b: Vec3::zeros(),
			material: Material::black(),
		}
	}
}
