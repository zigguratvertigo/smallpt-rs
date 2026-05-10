use crate::hit::Hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::{PrimitiveType, Traceable, Vec3};
use bvh::aabb::{Aabb, Bounded};
use bvh::bounding_hierarchy::BHShape;
use nalgebra::Point3;

#[derive(Copy, Clone)]
pub struct Triangle {
	pub p0: Vec3,
	pub p1: Vec3,
	pub p2: Vec3,
	pub normal: Vec3,
	pub n0: Vec3,
	pub n1: Vec3,
	pub n2: Vec3,
	pub material: Material,
	node_index: usize,
}

impl Triangle {
	pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, material: Material) -> Triangle {
		let normal = (p2 - p0).normalize().cross(&(p1 - p0).normalize());
		Triangle {
			p0,
			p1,
			p2,
			normal,
			n0: normal,
			n1: normal,
			n2: normal,
			material,
			node_index: 0,
		}
	}

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
			p0,
			p1,
			p2,
			n0,
			n1,
			n2,
			normal: (p2 - p0).normalize().cross(&(p1 - p0).normalize()),
			material,
			node_index: 0,
		}
	}
}

impl Traceable for Triangle {
	fn intersect(&self, r: &Ray, result: &mut Hit) -> bool {
		let p0p1 = self.p1 - self.p0;
		let p0p2 = self.p2 - self.p0;
		let pvec = r.direction.cross(&p0p2);

		let det = p0p1.dot(&pvec).abs();

		let tvec = r.origin - self.p0;
		let u = tvec.dot(&pvec) / det;
		if !(0.0..=1.0).contains(&u) {
			return false;
		}

		let qvec = tvec.cross(&p0p1);
		let v = r.direction.dot(&qvec) / det;
		if v < 0.0 || u + v > 1.0 {
			return false;
		}

		result.t = p0p2.dot(&qvec) / det;
		result.p = r.origin + r.direction * result.t;
		result.material = self.material;
		result.b = Vec3::new(1.0 - u - v, u, v);

		result.n = result.b.x * self.n0 + result.b.y * self.n1 + result.b.z * self.n2;

		true
	}

	fn get_primitive_type(&self) -> PrimitiveType {
		PrimitiveType::Triangle
	}
}

impl Bounded<f32, 3> for Triangle {
	fn aabb(&self) -> Aabb<f32, 3> {
		let min_x = self.p0.x.min(self.p1.x).min(self.p2.x);
		let min_y = self.p0.y.min(self.p1.y).min(self.p2.y);
		let min_z = self.p0.z.min(self.p1.z).min(self.p2.z);
		let max_x = self.p0.x.max(self.p1.x).max(self.p2.x);
		let max_y = self.p0.y.max(self.p1.y).max(self.p2.y);
		let max_z = self.p0.z.max(self.p1.z).max(self.p2.z);
		Aabb::with_bounds(
			Point3::new(min_x, min_y, min_z),
			Point3::new(max_x, max_y, max_z),
		)
	}
}

impl BHShape<f32, 3> for Triangle {
	fn set_bh_node_index(&mut self, index: usize) {
		self.node_index = index;
	}

	fn bh_node_index(&self) -> usize {
		self.node_index
	}
}
