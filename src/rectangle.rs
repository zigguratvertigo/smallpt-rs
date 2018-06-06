use Traceable;
use material::Material;
use plane::Plane;
use ray::Ray;

use cgmath::prelude::*;
extern crate cgmath;
type Float3 = cgmath::Vector3<f64>;

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub position: Float3,
    pub normal: Float3,
    pub left: Float3,
    pub up: Float3,
    pub width: f64,
    pub height: f64,
    pub material: Material,
}

impl Rectangle {
    // Spawn a new rectangle
    pub fn new(
        position: Float3,
        normal: Float3,
        left: Float3,
        up: Float3,
        width: f64,
        height: f64,
        material: Material,
    ) -> Rectangle {
        Rectangle {
            position: position,
            normal: normal,
            left: left,
            up: up,
            width: width,
            height: height,
            material: material,
        }
    }
}

impl Traceable for Rectangle {
    // Ray-Rectangle Intersection
    fn intersect(&self, r: &Ray) -> f64 {
        let rectangle_plane = Plane::new(self.position, self.normal, self.material);
        let intersect_plane_t = rectangle_plane.intersect(r);

        if intersect_plane_t > 0.0 {
            let p = r.origin + r.direction * intersect_plane_t;
            let v = p - self.position;

            let half_width = self.width * 0.5;
            let half_height = self.height * 0.5;

            // Project in 2D plane and clamp inside the rectangle
            if v.dot(self.left).abs() <= half_width && v.dot(self.up).abs() <= half_height {
                return intersect_plane_t;
            }
        }

        0.0
    }
}
