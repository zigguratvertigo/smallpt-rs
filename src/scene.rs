use bsdf::BSDF;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BoundingHierarchy;
use bvh::bvh::BVH;
use bvh::nalgebra::{Point3, Vector3};
use bvh::ray::Ray as NewRay;
use hit::Hit;
use ray::Ray;
use std::f32::*;
use triangle::Triangle;
use Traceable;

// #[derive(Default)]
pub struct Scene {
	pub objects: Vec<Box<Traceable>>,
	pub triangles: Vec<Triangle>,
}

impl Scene {
	pub fn add(&mut self, obj: Box<Traceable>) {
		self.objects.push(obj);
	}

	pub fn add_triangle(&mut self, triangle: Triangle) {
		self.triangles.push(triangle);
	}

	pub fn init() -> Scene {
		Scene {
			objects: vec![],
			triangles: vec![],
		}
	}

	pub fn intersect(&self, ray: Ray, bvh: &BVH) -> Option<Hit> {
		let mut final_hit = Hit::init();

		// Intersect parametric scene objects
		for s in 0..self.objects.len() {
			let mut current_hit = Hit::init();
			let hit = self.objects[s].intersect(&ray, &mut current_hit);

			// todo: hit min&max
			if hit == true && current_hit.t < final_hit.t && current_hit.t > 1e-6 {
				final_hit = current_hit;
			}
		}

		let bvh_ray = NewRay::new(
			Point3::new(ray.origin.x, ray.origin.y, ray.origin.z),
			ray.direction,
		);
		let hits = bvh.traverse(&bvh_ray, &self.triangles);

		// Triangles vs BVH
		if hits.len() > 0 {
			let mut current_hit = Hit::init();
			let hit = hits[0].intersect(&ray, &mut current_hit);

			if hit == true && current_hit.t < final_hit.t && current_hit.t > 1e-6 {
				final_hit = current_hit;
				// println!(
				// 	"We have a hit! {} {} {}",
				// 	final_hit.p.x, final_hit.p.y, final_hit.p.z
				// );
			}
		}

		if final_hit.t != INFINITY {
			Some(final_hit)
		} else {
			None
		}
	}

	pub fn build_bvh(&mut self) -> BVH {
		BVH::build(&mut self.triangles)
	}
}
