use ray::Ray;

use material::Material;

use cgmath::prelude::*;
extern crate cgmath;
type Float3 = cgmath::Vector3<f64>;

#[derive(Copy, Clone)]
pub struct Plane {
    pub position: Float3,
    pub normal: Float3,
    pub material: Material,
}

impl Plane {
    // Spawn a new plane
    pub fn new(position: Float3, normal: Float3, material: Material) -> Plane {
        Plane {
            position: position,
            normal: normal,
            material: material,
        }
    }

    // Ray-Plane Intersection
    pub fn intersect(self, ray: Ray) -> f64 {
        let plane_normal = -self.normal;
        let denom = plane_normal.dot(ray.direction);

        if denom > 1e-6 {
            plane_normal.dot(self.position - ray.origin) / denom
        } else {
            0.0
        }
    }
}
