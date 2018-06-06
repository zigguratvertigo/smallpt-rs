extern crate cgmath;
type Float3 = cgmath::Vector3<f64>;

use material::Material;

#[derive(Copy, Clone)]
pub struct Hit {
    pub position: Float3,
    pub normal: Float3,
    pub material: Material,
}

impl Hit {
    // Spawn a new Hit result data structure
    pub fn new(position: Float3, normal: Float3, material: Material) -> Hit {
        Hit {
            position: position,
            normal: normal,
            material: material,
        }
    }
}
