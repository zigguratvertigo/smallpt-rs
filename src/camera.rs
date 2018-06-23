// Basic camera
use vector::*;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub origin: Float3,
    pub forward: Float3,
    pub right: Float3,
    pub up: Float3,
}

impl Camera {
    pub fn new(
        origin: Float3,
        forward: Float3,    
        right: Float3,
        up: Float3) -> Camera
    {
        Camera {
            origin: origin,
            forward: forward,
            right: right,
            up: up
        }
    }
}