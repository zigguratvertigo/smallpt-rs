use crate::bsdf::BSDF;
use crate::Vec3;

#[derive(Copy, Clone)]
pub struct Material {
	pub emission: Vec3,
	pub albedo: Vec3,
	pub bsdf: BSDF,
}

impl Material {
	pub fn new(emission: Vec3, albedo: Vec3, bsdf: BSDF) -> Material {
		Material {
			emission,
			albedo,
			bsdf,
		}
	}

	pub fn black() -> Material {
		Material {
			emission: Vec3::new(0.0, 0.0, 0.0),
			albedo: Vec3::new(0.0, 0.0, 0.0),
			bsdf: BSDF::Diffuse,
		}
	}

	pub fn white() -> Material {
		Material {
			emission: Vec3::new(0.0, 0.0, 0.0),
			albedo: Vec3::new(1.0, 1.0, 1.0),
			bsdf: BSDF::Diffuse,
		}
	}
}
