extern crate cgmath;
type Float3 = cgmath::Vector3<f64>;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Float3,
    pub direction: Float3,
}

impl Ray {
    pub fn new(origin: Float3, direction: Float3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}
