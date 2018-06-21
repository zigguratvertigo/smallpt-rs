use std;
use Traceable;

use cgmath::prelude::*;
extern crate cgmath;
type Float3 = cgmath::Vector3<f64>;

use hit::Hit;
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
	fn intersect(&self, ray: &Ray, result: &mut Hit) -> bool {
		let op: Float3 = self.position - ray.origin;
		let b: f64 = op.dot(ray.direction);
		let det_sqrd: f64 = b * b - op.dot(op) + self.radius * self.radius;

		if det_sqrd <= 0.0 {
			return false;
		} else {
			let det = det_sqrd.sqrt();

			if b - det > 0.0 {
				result.t = b - det;
				result.p = ray.origin + ray.direction * result.t;
				result.n = (self.position - result.p).normalize();
				result.n = if Float3::dot(result.n, ray.direction) < 0.0 {
					result.n
				} else {
					result.n * -1.0
				};
				result.material = self.material;

				return true;
			} else if b + det > 0.0 {
				result.t = b + det;
				result.p = ray.origin + ray.direction * result.t;
				result.n = (self.position - result.p).normalize();
				result.n = if Float3::dot(result.n, ray.direction) < 0.0 {
					result.n
				} else {
					result.n * -1.0
				};
				result.material = self.material;

				return true;
			}

			return false;
		}
	}
}
