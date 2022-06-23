use bvh::Vector3;
use hit::Hit;
use material::Material;
use ray::Ray;
use std;
use PrimitiveType;
use Traceable;

#[derive(Copy, Clone)]
pub struct Sphere {
	pub radius: f32,
	pub position: Vector3,
	pub material: Material,
}

impl Sphere {
	// Spawn a new sphere
	pub fn new(radius: f32, position: Vector3, material: Material) -> Sphere {
		Sphere {
			radius,
			position,
			material,
		}
	}
}

impl Traceable for Sphere {
	fn intersect(&self, ray: &Ray, result: &mut Hit) -> bool {
		let op: Vector3 = self.position - ray.origin;
		let b: f32 = op.dot(ray.direction);
		let det_sqrd: f32 = b * b - op.dot(op) + self.radius * self.radius;

		if det_sqrd <= 0.0 {
			false
		} else {
			let det = det_sqrd.sqrt();

			if b - det > 0.01 {
				result.t = b - det;
				result.p = ray.origin + ray.direction * result.t;
				result.n = (self.position - result.p).normalize();
				result.n = if result.n.dot(ray.direction) < 0.0 {
					result.n
				} else {
					-result.n
				};
				result.material = self.material;

				return true;
			} else if b + det > 0.01 {
				result.t = b + det;
				result.p = ray.origin + ray.direction * result.t;
				result.n = (self.position - result.p).normalize();
				result.n = if result.n.dot(ray.direction) < 0.0 {
					result.n
				} else {
					-result.n
				};
				result.material = self.material;

				return true;
			}

			false
		}
	}

	fn get_primitive_type(&self) -> PrimitiveType {
		PrimitiveType::Sphere
	}
}
