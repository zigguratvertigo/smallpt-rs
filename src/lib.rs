//! ![uml](images/smallpt-uml.svg)

#![allow(unused_imports)]
#[macro_use]

extern crate log;
extern crate bvh;
extern crate num_cpus;
extern crate rand;
extern crate rayon;

use rand::prelude::*;
use rayon::prelude::*;
use std::f32::consts::PI;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod bsdf;
pub mod camera;
pub mod hit;
pub mod material;
pub mod plane;
pub mod ray;
pub mod rectangle;
pub mod scene;
pub mod sphere;
pub mod triangle;
pub mod vector;

pub use bsdf::*;
pub use bvh::*;
pub use camera::*;
pub use hit::*;
pub use material::*;
pub use plane::*;
pub use ray::*;
pub use rectangle::*;
pub use scene::*;
pub use sphere::*;
pub use triangle::*;

use bvh::bvh::BVH;

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveType {
	Triangle = 0,
	Plane = 1,
	Rectangle = 2,
	Sphere = 3,
}

pub trait Traceable: Send + Sync {
	fn intersect(&self, ray: &Ray, result: &mut Hit) -> bool;
	fn get_primitive_type(&self) -> PrimitiveType;
}

pub fn trace(
	scene: &Scene,
	camera: &Camera,
	width: usize,
	height: usize,
	samples: u32,
	backbuffer: &mut [Vector3],
	rays: &mut usize,
) {
	let ray_count = AtomicUsize::new(0);
	let inv_width = 1.0 / width as f32;
	let inv_height = 1.0 / height as f32;
	let inv_samples = 1.0 / samples as f32;

	// For each row of pixels
	backbuffer
		.par_chunks_mut(width as usize)
		.enumerate()
		.for_each(|(j, row)| {
			row.iter_mut().enumerate().for_each(|(i, output)| {
				let mut radiance = Vector3::new(0.0, 0.0, 0.0);
				let mut num_rays = 0;
				let mut rng = thread_rng();

				for _ in 0..samples {
					let rnd_x: f32 = rng.gen();
					let rnd_y: f32 = rng.gen();
					let dx = ((i as f32 + rnd_x) * inv_width) - 0.5;
					let dy = ((j as f32 + rnd_y) * inv_height) - 0.5;

					// Compute V
					let v = camera.forward + camera.right * dx - camera.up * dy;

					// Spawn a ray
					let ray = Ray {
						origin: camera.origin + v * 10.0,
						direction: v.normalize(),
					};

					radiance += compute_radiance(ray, &scene, 0, &mut num_rays);
				}

				ray_count.fetch_add(num_rays, Ordering::Relaxed);
				*output = radiance * inv_samples;
			});
		});

	*rays = ray_count.load(Ordering::Relaxed);
}

fn luminance(color: Vector3) -> f32 {
	0.299 * color.x + 0.587 * color.y + 0.114 * color.z
}

fn compute_radiance(ray: Ray, scene: &Scene, depth: i32, num_rays: &mut usize) -> Vector3 {
	*num_rays += 1;
	let intersect: Option<Hit> = scene.intersect(ray);

	match intersect {
		None => Vector3::new(0.0, 0.0, 0.0),
		Some(hit) => {
			let position = ray.origin + ray.direction * hit.t;
			let normal = hit.n;

			let mut f = hit.material.albedo;
			if depth > 3 {
				if rand::random::<f32>() < luminance(f) && depth < 10 {
					f = f / luminance(f);
				} else {
					return hit.material.emission;
				}
			}

			let irradiance: Vector3 = match hit.material.bsdf {
				// Diffuse Reflection
				BSDF::Diffuse => {
					// Sample cosine distribution and transform into world tangent space
					let r1 = 2.0 * PI * rand::random::<f32>();
					let r2 = rand::random::<f32>();
					let r2s = r2.sqrt();
					let w_up = if normal.x.abs() > 0.1 {
						Vector3::new(0.0, 1.0, 0.0)
					} else {
						Vector3::new(1.0, 0.0, 0.0)
					};

					let tangent = normal.cross(w_up).normalize();
					let bitangent = normal.cross(tangent).normalize();
					let next_direction = tangent * r1.cos() * r2s
						+ bitangent * r1.sin() * r2s
						+ normal * (1.0 - r2).sqrt();

					compute_radiance(
						Ray::new(position, next_direction.normalize()),
						scene,
						depth + 1,
						num_rays,
					)
				}

				// Mirror Reflection
				BSDF::Mirror => {
					let r = ray.direction.normalize()
						- normal.normalize() * 2.0 * normal.dot(ray.direction);

					compute_radiance(
						Ray::new(position, r.normalize()),
						scene,
						depth + 1,
						num_rays,
					)
				}

				// Glass / Translucent
				BSDF::Glass => {
					let r = ray.direction.normalize()
						- normal.normalize() * 2.0 * normal.dot(ray.direction);
					let reflection = Ray::new(position, r);

					// Compute input-output IOR
					let into = normal.dot(normal) > 0.0;
					let nc = 1.0;
					let nt = 1.5;
					let nnt = if into { nc / nt } else { nt / nc };

					// Compute fresnel
					let ddn = ray.direction.dot(normal);
					let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);

					if cos2t < 0.0 {
						// Total internal reflection
						compute_radiance(reflection, scene, depth + 1, num_rays)
					} else {
						let transmitted_dir = (ray.direction * nnt
							- normal * (if into { 1.0 } else { -1.0 } * (ddn * nnt + cos2t.sqrt())))
							.normalize();
						let transmitted_ray = Ray::new(position, transmitted_dir);

						let a = nt - nc;
						let b = nt + nc;
						let base_reflectance = a * a / (b * b);
						let c = 1.0 - if into {
							-ddn
						} else {
							transmitted_dir.dot(normal)
						};

						let reflectance =
							base_reflectance + (1.0 - base_reflectance) * c * c * c * c * c;
						let transmittance = 1.0 - reflectance;
						let rr_propability = 0.25 + 0.5 * reflectance;
						let reflectance_propability = reflectance / rr_propability;
						let transmittance_propability = transmittance / (1.0 - rr_propability);

						if depth > 1 {
							// Russian roulette between reflectance and transmittance
							if rand::random::<f32>() < rr_propability {
								compute_radiance(reflection, scene, depth + 1, num_rays)
									* reflectance_propability
							} else {
								compute_radiance(transmitted_ray, scene, depth + 1, num_rays)
									* transmittance_propability
							}
						} else {
							compute_radiance(reflection, scene, depth + 1, num_rays) * reflectance
								+ compute_radiance(transmitted_ray, scene, depth + 1, num_rays)
									* transmittance
						}
					}
				}
			};

			return irradiance * f + hit.material.emission;
		}
	}
}

// todo: remove me later

pub fn saturate(color: Vector3) -> Vector3 {
	Vector3::new(
		color.x.max(0.0).min(1.0),
		color.y.max(0.0).min(1.0),
		color.z.max(0.0).min(1.0),
	)
}

pub fn tonemap(color: Vector3) -> Vector3 {
	let color_linear = Vector3::new(
		color.x.powf(1.0 / 2.2),
		color.y.powf(1.0 / 2.2),
		color.z.powf(1.0 / 2.2),
	);

	return saturate(color_linear);
}
