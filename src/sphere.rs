use std;
use Traceable;

use cgmath::prelude::*;
extern crate cgmath;
type Float3 = cgmath::Vector3<f64>;

use material::Material;
use ray::Ray;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub radius: f64,
    pub position: Float3,
    pub material: Material,
}

impl Sphere {
    // Spawn a new sphere
    pub fn new(radius: f64, position: Float3, material: Material) -> Sphere {
        Sphere {
            radius: radius,
            position: position,
            material: material,
        }
    }
}

impl Traceable for Sphere {
    fn intersect(&self, ray: &Ray) -> f64 {
        let op: Float3 = self.position - ray.origin;
        let b: f64 = op.dot(ray.direction);
        let det_sqrd: f64 = b * b - op.dot(op) + self.radius * self.radius;

        if det_sqrd > 0.0 {
            let det = det_sqrd.sqrt();
            if b - det > 0.0 {
                b - det
            } else if b + det > 0.0 {
                b + det
            } else {
                std::f64::INFINITY
            }
        } else {
            std::f64::INFINITY
        }
    }
}
