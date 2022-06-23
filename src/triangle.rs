use bvh::{Point3, Vector3};
use hit::Hit;
use material::Material;
use ray::Ray;
use PrimitiveType;
use Traceable;

use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::{BHShape, BoundingHierarchy};
use bvh::bvh::BVH;

#[derive(Copy, Clone)]
pub struct Triangle {
	pub p0: Vector3,
	pub p1: Vector3,
	pub p2: Vector3,
	pub normal: Vector3,
	pub n0: Vector3,
	pub n1: Vector3,
	pub n2: Vector3,
	//
	pub material: Material,
	//
	aabb: AABB,
	node_index: usize,
}

impl Triangle {
	// Spawn a new rectangle
	pub fn new(p0: Vector3, p1: Vector3, p2: Vector3, material: Material) -> Triangle {
		Triangle {
			p0,
			p1,
			p2,
			normal: (p2 - p0).normalize().cross((p1 - p0).normalize()),
			n0: (p2 - p0).normalize().cross((p1 - p0).normalize()),
			n1: (p2 - p0).normalize().cross((p1 - p0).normalize()),
			n2: (p2 - p0).normalize().cross((p1 - p0).normalize()),
			material,
			aabb: AABB::empty().grow(&p0).grow(&p1).grow(&p2),
			node_index: 0,
		}
	}
	// Spawn a new rectangle, with per-vertex normals
	pub fn new_ext(
		p0: Vector3,
		p1: Vector3,
		p2: Vector3,
		n0: Vector3,
		n1: Vector3,
		n2: Vector3,
		material: Material,
	) -> Triangle {
		Triangle {
			p0,
			p1,
			p2,
			n0,
			n1,
			n2,
			normal: (p2 - p0).normalize().cross((p1 - p0).normalize()),
			material,
			aabb: AABB::empty().grow(&p0).grow(&p1).grow(&p2),
			node_index: 0,
		}
	}
}

impl Traceable for Triangle {
	// Ray-Triangle Intersection
	fn intersect(&self, r: &Ray, result: &mut Hit) -> bool {
		let p0p1 = self.p1 - self.p0;
		let p0p2 = self.p2 - self.p0;
		let pvec = r.direction.cross(p0p2);

		// if the determinant is negative the triangle is backfacing
		// if the determinant is close to 0, the ray misses the triangle
		let det = p0p1.dot(pvec).abs(); // double-sided
								// if det < 1e-6 {
								// 	return false;
								// }

		let tvec = r.origin - self.p0;
		let u = tvec.dot(pvec) / det;
		if !(0.0..=1.0).contains(&u) {
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
		result.b = Vector3::new(1.0 - u - v, u, v);

		// Compute interpolated normal
		result.n = result.b.x * self.n0 + result.b.y * self.n1 + result.b.z * self.n2;

		true
	}

	fn get_primitive_type(&self) -> PrimitiveType {
		PrimitiveType::Triangle
	}
}

impl Bounded for Triangle {
	fn aabb(&self) -> AABB {
		self.aabb
	}
}

impl BHShape for Triangle {
	fn set_bh_node_index(&mut self, index: usize) {
		self.node_index = index;
	}

	fn bh_node_index(&self) -> usize {
		self.node_index
	}
}
