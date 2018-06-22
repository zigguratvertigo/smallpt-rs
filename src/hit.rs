use material::Material;
use std::f32::*;
use vector::*;

#[derive(Copy, Clone)]
pub struct Hit {
    pub p: Float3,
    pub n: Float3,
    pub t: f32,
    pub b: Float3,
    pub material: Material,
}

impl Hit {
    // Spawn a new Hit result data structure
    pub fn new(p: Float3, n: Float3, t: f32, b: Float3, material: Material) -> Hit {
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
            b: Float3::zero(),
            material: Material::black(),
        }
    }
}
