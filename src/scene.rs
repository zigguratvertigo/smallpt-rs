use crate::hit::Hit;
use crate::ray::Ray;
use crate::triangle::Triangle;
use crate::Traceable;
use bvh::bvh::Bvh;
use bvh::ray::Ray as BvhRay;
use nalgebra::{Point3, Vector3};
use std::f32::INFINITY;

pub struct Scene {
	pub objects: Vec<Box<dyn Traceable>>,
	pub triangles: Vec<Triangle>,
	bvh: Bvh<f32, 3>,
	bvh_built: bool,
}

impl Scene {
	pub fn add(&mut self, obj: Box<dyn Traceable>) {
		self.objects.push(obj);
	}

	pub fn add_triangle(&mut self, triangle: Triangle) {
		self.triangles.push(triangle);
	}

	pub fn init() -> Scene {
		Scene {
			objects: vec![],
			triangles: vec![],
			bvh: Bvh { nodes: vec![] },
			bvh_built: false,
		}
	}

	pub fn intersect(&self, ray: Ray) -> Option<Hit> {
		let mut final_hit = Hit::init();

		for s in 0..self.objects.len() {
			let mut current_hit = Hit::init();
			let hit = self.objects[s].intersect(&ray, &mut current_hit);

			if hit && current_hit.t < final_hit.t && current_hit.t > 1e-6 {
				final_hit = current_hit;
			}
		}

		if self.bvh_built {
			let bvh_ray = BvhRay::new(
				Point3::new(ray.origin.x, ray.origin.y, ray.origin.z),
				Vector3::new(ray.direction.x, ray.direction.y, ray.direction.z),
			);
			let hits = self.bvh.traverse(&bvh_ray, &self.triangles);

			if !hits.is_empty() {
				let mut current_hit = Hit::init();

				for hit in hits {
					let is_hit = hit.intersect(&ray, &mut current_hit);

					if is_hit && current_hit.t < final_hit.t && current_hit.t > 1e-6 {
						final_hit = current_hit;
					}
				}
			}
		}

		if final_hit.t != INFINITY {
			Some(final_hit)
		} else {
			None
		}
	}

	pub fn build_bvh(&mut self) {
		self.bvh = Bvh::build(&mut self.triangles);
		self.bvh_built = true;
	}
}
