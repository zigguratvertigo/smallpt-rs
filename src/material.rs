use bsdf::BSDF;
use vector::*;

#[derive(Copy, Clone)]
pub struct Material {
    pub emission: Float3,
    pub albedo: Float3,
    pub bsdf: BSDF,
}

impl Material {
    pub fn new(emission: Float3, albedo: Float3, bsdf: BSDF) -> Material {
        Material {
            emission: emission,
            albedo: albedo,
            bsdf: bsdf,
        }
    }

    pub fn black() -> Material {
        Material {
            emission: Float3::zero(),
            albedo: Float3::zero(),
            bsdf: BSDF::Diffuse,
        }
    }

    pub fn white() -> Material {
        Material {
            emission: Float3::zero(),
            albedo: Float3::new(1.0, 1.0, 1.0),
            bsdf: BSDF::Diffuse,
        }
    }
}
