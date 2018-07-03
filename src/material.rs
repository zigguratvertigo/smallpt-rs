use bsdf::BSDF;
use nalgebra::{Vector3};
type Vec3 = Vector3<f32>;

#[derive(Copy, Clone)]
pub struct Material {
	pub emission: Vec3,
	pub albedo: Vec3,
	pub bsdf: BSDF,
}

impl Material {
	pub fn new(emission: Vec3, albedo: Vec3, bsdf: BSDF) -> Material {
		Material {
			emission: emission,
			albedo: albedo,
			bsdf: bsdf,
		}
	}

	pub fn black() -> Material {
		Material {
			emission: Vec3::zeros(),
			albedo: Vec3::zeros(),
			bsdf: BSDF::Diffuse,
		}
	}

	pub fn white() -> Material {
		Material {
			emission: Vec3::zeros(),
			albedo: Vec3::new(1.0, 1.0, 1.0),
			bsdf: BSDF::Diffuse,
		}
	}
}
