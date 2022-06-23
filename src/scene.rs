use bsdf::BSDF;
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BoundingHierarchy;
use bvh::bvh::BVH;
use bvh::ray::Ray as NewRay;
use bvh::Point3;
use hit::Hit;
use ray::Ray;
use std::f32::INFINITY;
use triangle::Triangle;
use Traceable;

// #[derive(Default)]
pub struct Scene {
	pub objects: Vec<Box<dyn Traceable>>,
	pub triangles: Vec<Triangle>,
	//
	bvh: BVH,
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
			bvh: BVH { nodes: vec![] },
			bvh_built: false,
		}
	}

	pub fn intersect(&self, ray: Ray) -> Option<Hit> {
		let mut final_hit = Hit::init();

		// Intersect parametric scene objects
		for s in 0..self.objects.len() {
			let mut current_hit = Hit::init();
			let hit = self.objects[s].intersect(&ray, &mut current_hit);

			// todo: hit min&max
			if hit && current_hit.t < final_hit.t && current_hit.t > 1e-6 {
				final_hit = current_hit;
			}
		}

		if self.bvh_built {
			let bvh_ray = NewRay::new(
				Point3::new(ray.origin.x, ray.origin.y, ray.origin.z),
				Point3::new(ray.direction.x, ray.direction.y, ray.direction.z),
			);
			let hits = self.bvh.traverse(&bvh_ray, &self.triangles);

			// Triangles vs BVH
			if !hits.is_empty() {
				let mut current_hit = Hit::init();

				// Of all the hits, return the closest hit
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
		self.bvh = BVH::build(&mut self.triangles);
		self.bvh_built = true; //boo
	}
}
