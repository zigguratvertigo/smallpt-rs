use hit::Hit;
use material::Material;
use nalgebra::{Point3, Vector3};
use ray::Ray;
use PrimitiveType;
use Traceable;

type Vec3 = Vector3<f32>;
type Pt3 = Point3<f32>;

use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::{BHShape, BoundingHierarchy};
use bvh::bvh::BVH;
use std::simd::f32x4;

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
	//
	aabb: AABB,
	node_index: usize,
}

impl Triangle {
	// Spawn a new rectangle
	pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, material: Material) -> Triangle {
		let temp_p0 = f32x4::new(p0.x, p0.y, p0.z, 0.0f32);
		let temp_p1 = f32x4::new(p1.x, p1.y, p1.z, 0.0f32);
		let temp_p2 = f32x4::new(p2.x, p2.y, p2.z, 0.0f32);

		Triangle {
			p0: p0,
			p1: p1,
			p2: p2,
			normal: (p2 - p0).normalize().cross(&(&p1 - &p0).normalize()),
			n0: (p2 - p0).normalize().cross(&(&p1 - &p0).normalize()),
			n1: (p2 - p0).normalize().cross(&(&p1 - &p0).normalize()),
			n2: (p2 - p0).normalize().cross(&(&p1 - &p0).normalize()),
			material: material,
			aabb: AABB::empty().grow(temp_p0).grow(temp_p1).grow(temp_p2),
			node_index: 0,
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
		let temp_p0 = f32x4::new(p0.x, p0.y, p0.z, 0.0f32);
		let temp_p1 = f32x4::new(p1.x, p1.y, p1.z, 0.0f32);
		let temp_p2 = f32x4::new(p2.x, p2.y, p2.z, 0.0f32);

		Triangle {
			p0: p0,
			p1: p1,
			p2: p2,
			n0: n0,
			n1: n1,
			n2: n2,
			normal: (p2 - p0).normalize().cross(&(&p1 - &p0).normalize()),
			material: material,
			aabb: AABB::empty().grow(temp_p0).grow(temp_p1).grow(temp_p2),
			node_index: 0,
		}
	}
}

impl Traceable for Triangle {
	// Ray-Triangle Intersection
	fn intersect(&self, r: &Ray, result: &mut Hit) -> bool {
		let p0p1 = self.p1 - self.p0;
		let p0p2 = self.p2 - self.p0;
		let pvec = r.direction.cross(&p0p2);

		// if the determinant is negative the triangle is backfacing
		// if the determinant is close to 0, the ray misses the triangle
		let det = p0p1.dot(&pvec).abs(); // double-sided
								   // if det < 1e-6 {
								   // 	return false;
								   // }

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
		result.n = result.b.x * self.n0 + result.b.y * self.n1 + result.b.z * self.n2;

		return true;
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
	// fn set_bh_node_index(&mut self, index: usize) {
	// 	self.node_index = index;
	// }

	// fn bh_node_index(&self) -> usize {
	// 	self.node_index
	// }
}
