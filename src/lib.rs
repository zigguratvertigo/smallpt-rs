#![allow(unused_imports)]

extern crate cgmath;
extern crate num_cpus;
extern crate rand;
extern crate rayon;

use std::f64::consts::PI;
use cgmath::prelude::*;
use rayon::prelude::*;
use std::path::PathBuf;

type Float3 = cgmath::Vector3<f64>;

pub mod plane;
pub mod rectangle;
pub mod sphere;
pub mod triangle;
pub mod ray;
pub mod scene;
pub mod hit;
pub mod bsdf;
pub mod material;

pub use plane::*;
pub use rectangle::*;
pub use sphere::*;
pub use triangle::*;
pub use ray::*;
pub use scene::*;
pub use hit::*;
pub use bsdf::*;
pub use material::*;

trait Traceable : Send + Sync {
    fn intersect(&self, ray: &Ray) -> f64;
}

pub fn trace(scene: &Scene, camera: &Ray, width: usize, height: usize, num_samples: u32, backbuffer: &mut [Float3]) {
    let aperture = 0.5135;
    let cx = Float3::new(width as f64 * aperture / height as f64, 0.0, 0.0);
    let cy = cx.cross(camera.direction).normalize() * aperture;
    
    // Split the work
    let num_cpus = num_cpus::get();
    let num_inner_chunks = num_cpus * num_cpus;
    let num_outer_chunks = 100;
    let outer_chunk_size = ceil_divide(width * height, num_outer_chunks);

    for (outer_chunk_index, outer_chunk) in backbuffer.chunks_mut(outer_chunk_size).enumerate() {
        let inner_chunk_size = ceil_divide(outer_chunk_size, num_inner_chunks);

        // Create and process parallel outer chunks
        outer_chunk
            .par_chunks_mut(inner_chunk_size)
            .enumerate()
            .for_each(|(inner_chunk_index, inner_chunk)| {
                for i in 0..inner_chunk.len() {
                    let pixel_index = i + inner_chunk_index * inner_chunk_size + outer_chunk_index * outer_chunk_size;
                    let x = (pixel_index % width) as f64;
                    let y = (height - pixel_index / width - 1) as f64;

                    let mut radiance = Float3::zero();

                    // Samples per pixel
                    for _ in 0..num_samples {

                        // Jitter for AA
                        let r1: f64 = 2.0 * rand::random::<f64>();
                        let dx = if r1 < 1.0 {
                            r1.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r1).sqrt()
                        };
                        let r2: f64 = 2.0 * rand::random::<f64>();
                        let dy = if r2 < 1.0 {
                            r2.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r2).sqrt()
                        };
                        
                        // Compute V
                        let v = camera.direction
                            + cx * (((0.5 + dx) / 2.0 + x) / width as f64 - 0.5)
                            + cy * (((0.5 + dy) / 2.0 + y) / height as f64 - 0.5);

                        // Spawn a ray
                        let ray = Ray {
                            origin: camera.origin + v * 100.0,
                            direction: v.normalize(),
                        };

                        radiance += compute_radiance(ray, &scene, 0);
                    }

                    inner_chunk[i] = radiance / (num_samples as f64);
                }
            });
        
        println!("Rendering ({} spp) {}%\r", num_samples, outer_chunk_index);
    }
}

fn luminance(color: Float3) -> f64 {
    0.299 * color.x + 0.587 * color.y + 0.114 * color.z
}

fn compute_radiance(ray: Ray, scene: &Scene, depth: i32) -> Float3 {
    let intersect: Option<(Hit, f64)> = scene.intersect(ray);

    match intersect {
        None => Float3::zero(),
        Some((hit, t)) => {
            let position = ray.origin + ray.direction * t;
            let normal = hit.normal;

            let mut f = hit.material.albedo;
            if depth > 3 {
                if rand::random::<f64>() < luminance(f) && depth < 100 {
                    f = f / luminance(f);
                } else {
                    return hit.material.emission;
                }
            }

            let irradiance: Float3 = match hit.material.bsdf {
                // Diffuse Reflection
                BSDF::Diffuse => {
                    // Sample cosine distribution and transform into world tangent space
                    let r1 = 2.0 * PI * rand::random::<f64>();
                    let r2 = rand::random::<f64>();
                    let r2s = r2.sqrt();
                    let w_up = if normal.x.abs() > 0.1 {
                        Float3::new(0.0, 1.0, 0.0)
                    } else {
                        Float3::new(1.0, 0.0, 0.0)
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
                    )
                }

                // Mirror Reflection
                BSDF::Mirror => {
                    let r = ray.direction - normal * 2.0 * normal.dot(ray.direction);

                    compute_radiance(Ray::new(position, r), scene, depth + 1)
                }

                // Glass / Translucent
                BSDF::Glass => {
                    let r = ray.direction - normal * 2.0 * normal.dot(ray.direction);
                    let reflection = Ray::new(position, r);

                    // Compute input-output IOR
                    let into = Float3::dot(normal, normal) > 0.0;
                    let nc = 1.0;
                    let nt = 1.5;
                    let nnt = if into { nc / nt } else { nt / nc };

                    // Compute fresnel
                    let ddn = Float3::dot(ray.direction, normal);
                    let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);

                    if cos2t < 0.0 {
                        // Total internal reflection
                        compute_radiance(reflection, scene, depth + 1)
                    } else {
                        let transmitted_dir =
                            (ray.direction * nnt
                                - normal
                                    * (if into { 1.0 } else { -1.0 } * (ddn * nnt + cos2t.sqrt())))
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

                        let reflectance = base_reflectance + (1.0 - base_reflectance) * c * c * c * c * c;
                        let transmittance = 1.0 - reflectance;
                        let rr_propability = 0.25 + 0.5 * reflectance;
                        let reflectance_propability = reflectance / rr_propability;
                        let transmittance_propability = transmittance / (1.0 - rr_propability);

                        if depth > 1 {
                            // Russian roulette between reflectance and transmittance
                            if rand::random::<f64>() < rr_propability {
                                compute_radiance(reflection, scene, depth + 1) * reflectance_propability
                            } else {
                                compute_radiance(transmitted_ray, scene, depth + 1) * transmittance_propability
                            }
                        } else {
                            compute_radiance(reflection, scene, depth + 1) * reflectance
                                + compute_radiance(transmitted_ray, scene, depth + 1) * transmittance
                        }
                    }
                }
            };

            return Float3::new(
                irradiance.x * f.x + hit.material.emission.x,
                irradiance.y * f.y + hit.material.emission.y,
                irradiance.z * f.z + hit.material.emission.z,
            );
        }
    }
}

fn ceil_divide(dividend: usize, divisor: usize) -> usize {
    let division = dividend / divisor;
    if division * divisor == dividend {
        division
    } else {
        division + 1
    }
}