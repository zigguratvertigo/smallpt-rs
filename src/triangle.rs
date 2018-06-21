use hit::Hit;
use material::Material;
use ray::Ray;
use Traceable;

use cgmath::prelude::*;
extern crate cgmath;
type Float2 = cgmath::Vector2<f64>;
type Float3 = cgmath::Vector3<f64>;

#[derive(Copy, Clone)]
pub struct Triangle {
	pub p0: Float3,
	pub p1: Float3,
	pub p2: Float3,
	pub normal: Float3,
	pub n0: Float3,
	pub n1: Float3,
	pub n2: Float3,
	pub material: Material,
}

impl Triangle {
	// Spawn a new rectangle
	pub fn new(p0: Float3, p1: Float3, p2: Float3, material: Material) -> Triangle {
		Triangle {
			p0: p0,
			p1: p1,
			p2: p2,
			normal: (p2 - p0).normalize().cross((p1 - p0).normalize()),
			n0: (p2 - p0).normalize().cross((p1 - p0).normalize()),
			n1: (p2 - p0).normalize().cross((p1 - p0).normalize()),
			n2: (p2 - p0).normalize().cross((p1 - p0).normalize()),
			material: material,
		}
	}
	// Spawn a new rectangle, with per-vertex normals
	pub fn new_ext(p0: Float3, p1: Float3, p2: Float3, n0: Float3, n1: Float3, n2: Float3, material: Material) -> Triangle {
		Triangle {
			p0: p0,
			p1: p1,
			p2: p2,
			n0: n0,
			n1: n1,
			n2: n2,
			normal: (p2 - p0).normalize().cross((p1 - p0).normalize()),
			material: material,
		}
	}
}

impl Traceable for Triangle {
	// Ray-Triangle Intersection
	fn intersect(&self, r: &Ray, result: &mut Hit) -> bool {
		let p0p1 = self.p1 - self.p0;
		let p0p2 = self.p2 - self.p0;
		let pvec = r.direction.cross(p0p2);
		let det = p0p1.dot(pvec);

		// if the determinant is negative the triangle is backfacing
		// if the determinant is close to 0, the ray misses the triangle
		if det < 1e-6 {
			return false;
		}

		let tvec = r.origin - self.p0;
		let u = tvec.dot(pvec) / det;
		if u < 0.0 || u > 1.0 {
			return false;
		};

		let qvec = tvec.cross(p0p1);
		let v = r.direction.dot(qvec) / det;
		if v < 0.0 || u + v > 1.0 {
			return false;
		};

		// intersection
		result.t = p0p2.dot(qvec) / det;
		result.p = r.origin + r.direction * result.t;
		result.material = self.material;

		// Compute interpolated normals
		let w = 1.0 - u - v;
		result.n = w*self.n0 + u*self.n1 + v*self.n2;
		result.b = Float2::new(u, v);

		return true;
	}
}
