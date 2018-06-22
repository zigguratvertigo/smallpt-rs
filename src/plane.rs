use material::Material;
use ray::Ray;
use vector::*;
use Hit;
use Traceable;

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
}

impl Traceable for Plane {
    // Ray-Plane Intersection
    fn intersect(&self, r: &Ray, result: &mut ::Hit) -> bool {
        let plane_normal = -self.normal;
        let denom = plane_normal.dot(r.direction);

        if denom > 1e-6 {
            result.t = plane_normal.dot(self.position - r.origin) / denom;
            result.p = r.origin + r.direction * result.t;
            result.n = if Float3::dot(self.normal, r.direction) < 0.0 {
                self.normal
            } else {
                -self.normal
            };
            result.material = self.material;

            return true;
        } else {
            return false;
        }
    }
}
