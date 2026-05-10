use crate::hit::Hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::{PrimitiveType, Traceable, Vec3};

#[derive(Copy, Clone)]
pub struct Plane {
	pub position: Vec3,
	pub normal: Vec3,
	pub material: Material,
}

impl Plane {
	pub fn new(position: Vec3, normal: Vec3, material: Material) -> Plane {
		Plane {
			position,
			normal,
			material,
		}
	}
}

impl Traceable for Plane {
	fn intersect(&self, r: &Ray, result: &mut Hit) -> bool {
		let plane_normal = -self.normal;
		let denom = plane_normal.dot(&r.direction);

		if denom > 1e-6 {
			result.t = plane_normal.dot(&(self.position - r.origin)) / denom;
			result.p = r.origin + r.direction * result.t;
			result.n = if self.normal.dot(&r.direction) < 0.0 {
				self.normal
			} else {
				-self.normal
			};
			result.material = self.material;

			true
		} else {
			false
		}
	}

	fn get_primitive_type(&self) -> PrimitiveType {
		PrimitiveType::Plane
	}
}
