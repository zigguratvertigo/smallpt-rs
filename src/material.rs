use bsdf::BSDF;
use bvh::*;

#[derive(Copy, Clone)]
pub struct Material {
	pub emission: Vector3,
	pub albedo: Vector3,
	pub bsdf: BSDF,
}

impl Material {
	pub fn new(emission: Vector3, albedo: Vector3, bsdf: BSDF) -> Material {
		Material {
			emission: emission,
			albedo: albedo,
			bsdf: bsdf,
		}
	}

	pub fn black() -> Material {
		Material {
			emission: Vector3::new(0.0, 0.0, 0.0),
			albedo: Vector3::new(0.0, 0.0, 0.0),
			bsdf: BSDF::Diffuse,
		}
	}

	pub fn white() -> Material {
		Material {
			emission: Vector3::new(0.0, 0.0, 0.0),
			albedo: Vector3::new(1.0, 1.0, 1.0),
			bsdf: BSDF::Diffuse,
		}
	}
}
