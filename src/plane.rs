use material::Material;
use nalgebra::Vector3;
use ray::Ray;
use Hit;
use PrimitiveType;
use Traceable;
type Vec3 = Vector3<f32>;

#[derive(Copy, Clone)]
pub struct Plane {
	pub position: Vec3,
	pub normal: Vec3,
	pub material: Material,
}

impl Plane {
	// Spawn a new plane
	pub fn new(position: Vec3, normal: Vec3, material: Material) -> Plane {
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

			return true;
		} else {
			return false;
		}
	}

	fn get_primitive_type(&self) -> PrimitiveType {
		PrimitiveType::Plane
	}
}
