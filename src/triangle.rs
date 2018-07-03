use hit::Hit;
use material::Material;
use ray::Ray;
use Traceable;
use nalgebra::{Vector3};
type Vec3 = Vector3<f32>;

#[derive(Copy, Clone)]
pub struct Triangle {
	pub p0: Vec3,
	pub p1: Vec3,
	pub p2: Vec3,
	pub normal: Vec3,
	pub n0: Vec3,
	pub n1: Vec3,
	pub n2: Vec3,
	//
	pub material: Material,
	// aabb: AABB,
}

impl Triangle {
	// Spawn a new rectangle
	pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, material: Material) -> Triangle {
		Triangle {
			p0: p0,
			p1: p1,
			p2: p2,
			normal: (p2 - p0).normalize().cross(&(&p1 - &p0).normalize()),
			n0: (p2 - p0).normalize().cross(&(&p1 - &p0).normalize()),
			n1: (p2 - p0).normalize().cross(&(&p1 - &p0).normalize()),
			n2: (p2 - p0).normalize().cross(&(&p1 - &p0).normalize()),
			material: material,
			// aabb: AABB::empty().grow(&p0).grow(&p1).grow(&p2),
		}
	}
	// Spawn a new rectangle, with per-vertex normals
	pub fn new_ext(
		p0: Vec3,
		p1: Vec3,
		p2: Vec3,
		n0: Vec3,
		n1: Vec3,
		n2: Vec3,
		material: Material,
	) -> Triangle {
		Triangle {
			p0: p0,
			p1: p1,
			p2: p2,
			n0: n0,
			n1: n1,
			n2: n2,
			normal: (p2 - p0).normalize().cross(&(&p1 - &p0).normalize()),
			material: material,
		}
	}
}

impl Traceable for Triangle {
	// Ray-Triangle Intersection
	fn intersect(&self, r: &Ray, result: &mut Hit) -> bool {
		let p0p1 = self.p1 - self.p0;
		let p0p2 = self.p2 - self.p0;
		let pvec = r.direction.cross(&p0p2);
		let det = p0p1.dot(&pvec);

		// if the determinant is negative the triangle is backfacing
		// if the determinant is close to 0, the ray misses the triangle
		if det < 1e-6 {
			return false;
		}

		let tvec = r.origin - self.p0;
		let u = tvec.dot(&pvec) / det;
		if u < 0.0 || u > 1.0 {
			return false;
		};

		let qvec = tvec.cross(&p0p1);
		let v = r.direction.dot(&qvec) / det;
		if v < 0.0 || u + v > 1.0 {
			return false;
		};

		// intersection
		result.t = p0p2.dot(&qvec) / det;
		result.p = r.origin + r.direction * result.t;
		result.material = self.material;
		result.b = Vec3::new(1.0 - u - v, u, v);

		// Compute interpolated normal
		result.n =
			result.b.x * self.n0 + result.b.y * self.n1 + result.b.z * self.n2;

		return true;
	}
}
