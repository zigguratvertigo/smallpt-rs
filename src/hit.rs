extern crate cgmath;
type Float2 = cgmath::Vector2<f64>;
type Float3 = cgmath::Vector3<f64>;
pub use cgmath::prelude::*;

use material::Material;
use std::f64::*;

#[derive(Copy, Clone)]
pub struct Hit {
	pub p: Float3,
	pub n: Float3,
	pub t: f64,
	pub b: Float2,
	pub material: Material,
}

impl Hit {
	// Spawn a new Hit result data structure
	pub fn new(p: Float3, n: Float3, t: f64, b: Float2, material: Material) -> Hit {
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
			p: Float3::zero(),
			n: Float3::zero(),
			t: INFINITY,
			b: Float2::zero(),
			material: Material::black(),
		}
	}
}
