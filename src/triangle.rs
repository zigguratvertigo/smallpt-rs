use Traceable;
use material::Material;
use ray::Ray;

use cgmath::prelude::*;
extern crate cgmath;
type Float3 = cgmath::Vector3<f64>;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub p0: Float3,
    pub p1: Float3,
    pub p2: Float3,
    pub normal: Float3,
    pub material: Material,
}

impl Triangle {
    // Spawn a new rectangle
    pub fn new(
        p0: Float3,
        p1: Float3,
        p2: Float3,
        material: Material,
    ) -> Triangle {
        Triangle {
            p0: p0,
            p1: p1,
            p2: p2,
            normal: (p2 - p0).normalize().cross((p1 - p0).normalize()),
            material: material,
        }
    }
}

impl Traceable for Triangle {
    // Ray-Triangle Intersection
    fn intersect(&self, r: &Ray) -> f64 {
        let p0p1 = self.p1 - self.p0;
        let p0p2 = self.p2 - self.p0;
        let pvec = r.direction.cross(p0p2);
        let det = p0p1.dot(pvec);

        // if the determinant is negative the triangle is backfacing
        // if the determinant is close to 0, the ray misses the triangle

        if det < 1e-6 { return 0.0; }
   
        let tvec = r.origin - self.p0; 
        let u = tvec.dot(pvec) / det; 
        if u < 0.0 || u > 1.0 { return 0.0 }; 
    
        let qvec = tvec.cross(p0p1); 
        let v = r.direction.dot(qvec) / det; 
        if v < 0.0 || u + v > 1.0 { return 0.0 }; 

        // intersection
        return p0p2.dot(qvec) / det; 
    }
}
