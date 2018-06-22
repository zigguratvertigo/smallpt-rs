#![allow(unused_imports)]

#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate rand;
extern crate rayon;

use rayon::prelude::*;
use std::f32::consts::PI;
use std::path::PathBuf;

pub mod bsdf;
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
pub use hit::*;
pub use material::*;
pub use plane::*;
pub use ray::*;
pub use rectangle::*;
pub use scene::*;
pub use sphere::*;
pub use triangle::*;
pub use vector::*;

pub trait Traceable: Send + Sync {
    fn intersect(&self, ray: &Ray, result: &mut Hit) -> bool;
}

pub fn trace(
    scene: &Scene,
    camera: &Ray,
    width: usize,
    height: usize,
    num_samples: u32,
    backbuffer: &mut [Float3],
) {
    let aperture = 0.5135;
    let cx = Float3::new(width as f32 * aperture / height as f32, 0.0, 0.0);
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
                    let pixel_index = i
                        + inner_chunk_index * inner_chunk_size
                        + outer_chunk_index * outer_chunk_size;
                    let x = (pixel_index % width) as f32;
                    let y = (height - pixel_index / width - 1) as f32;

                    let mut radiance = Float3::zero();

                    // Samples per pixel
                    for _ in 0..num_samples {
                        // Jitter for AA
                        let r1: f32 = 2.0 * rand::random::<f32>();
                        let dx = if r1 < 1.0 {
                            r1.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r1).sqrt()
                        };
                        let r2: f32 = 2.0 * rand::random::<f32>();
                        let dy = if r2 < 1.0 {
                            r2.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r2).sqrt()
                        };

                        // Compute V
                        let v = camera.direction
                            + cx * (((0.5 + dx) / 2.0 + x) / width as f32 - 0.5)
                            + cy * (((0.5 + dy) / 2.0 + y) / height as f32 - 0.5);

                        // Spawn a ray
                        let ray = Ray {
                            origin: camera.origin + v * 10.0,
                            direction: v.normalize(),
                        };

                        radiance += compute_radiance(ray, &scene, 0);
                    }

                    inner_chunk[i] = radiance / num_samples as f32;
                }
            });

        debug!("Rendering ({} spp) {}%\r", num_samples, outer_chunk_index);
    }
}

fn luminance(color: Float3) -> f32 {
    0.299 * color.get_x() + 0.587 * color.get_y() + 0.114 * color.get_z()
}

fn compute_radiance(ray: Ray, scene: &Scene, depth: i32) -> Float3 {
    let intersect: Option<Hit> = scene.intersect(ray);

    match intersect {
        None => Float3::zero(),
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

            let irradiance: Float3 = match hit.material.bsdf {
                // Diffuse Reflection
                BSDF::Diffuse => {
                    // Sample cosine distribution and transform into world tangent space
                    let r1 = 2.0 * PI * rand::random::<f32>();
                    let r2 = rand::random::<f32>();
                    let r2s = r2.sqrt();
                    let w_up = if normal.get_x().abs() > 0.1 {
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
                    let r = ray.direction.normalize() - normal.normalize() * 2.0 * normal.dot(ray.direction);

                    compute_radiance(Ray::new(position, r.normalize()), scene, depth + 1)
                }

                // Glass / Translucent
                BSDF::Glass => {
                    let r = ray.direction.normalize() - normal.normalize() * 2.0 * normal.dot(ray.direction);
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
                        compute_radiance(reflection, scene, depth + 1)
                    } else {
                        let transmitted_dir = (ray.direction * nnt
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

                        let reflectance =
                            base_reflectance + (1.0 - base_reflectance) * c * c * c * c * c;
                        let transmittance = 1.0 - reflectance;
                        let rr_propability = 0.25 + 0.5 * reflectance;
                        let reflectance_propability = reflectance / rr_propability;
                        let transmittance_propability = transmittance / (1.0 - rr_propability);

                        if depth > 1 {
                            // Russian roulette between reflectance and transmittance
                            if rand::random::<f32>() < rr_propability {
                                compute_radiance(reflection, scene, depth + 1)
                                    * reflectance_propability
                            } else {
                                compute_radiance(transmitted_ray, scene, depth + 1)
                                    * transmittance_propability
                            }
                        } else {
                            compute_radiance(reflection, scene, depth + 1) * reflectance
                                + compute_radiance(transmitted_ray, scene, depth + 1)
                                    * transmittance
                        }
                    }
                }
            };

            //return irradiance * f;

            return Float3::new(
                irradiance.get_x() * f.get_x() + hit.material.emission.get_x(),
                irradiance.get_y() * f.get_y() + hit.material.emission.get_y(),
                irradiance.get_z() * f.get_z() + hit.material.emission.get_z(),
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

// todo: remove me later

pub fn saturate(color: Float3) -> Float3 {
    Float3::new(
        color.get_x().max(0.0).min(1.0),
        color.get_y().max(0.0).min(1.0),
        color.get_z().max(0.0).min(1.0),
    )
}

pub fn tonemap(color: Float3) -> Float3 {
    let color_linear = Float3::new(
        color.get_x().powf(1.0 / 2.2),
        color.get_y().powf(1.0 / 2.2),
        color.get_z().powf(1.0 / 2.2),
    );

    return saturate(color_linear);
}
