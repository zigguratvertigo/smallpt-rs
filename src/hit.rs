extern crate cgmath;
type Float3 = cgmath::Vector3<f64>;
pub use cgmath::prelude::*;

use material::Material;
use std::f64::*;

#[derive(Copy, Clone)]
pub struct Hit {
    pub p: Float3,
    pub n: Float3,
    pub t: f64,
    pub material: Material,
}

impl Hit {
    // Spawn a new Hit result data structure
    pub fn new(p: Float3, n: Float3, t: f64, material: Material) -> Hit {
        Hit {
            p: p,
            n: n,
            t: t,
            material: material,
        }
    }

    pub fn init() -> Hit {
        Hit {
            p: Float3::zero(),
            n: Float3::zero(),
            t: INFINITY,
            material: Material::black(),
        }
    }
}
