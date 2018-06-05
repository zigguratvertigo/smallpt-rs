#![allow(unused_imports)]
#![allow(dead_code)]

extern crate cgmath;
extern crate minifb;
extern crate num_cpus;
extern crate rand;
extern crate rayon;

#[macro_use]
extern crate structopt;

use std::f64::consts::PI;

mod bsdf;
mod intersection;
mod material;
mod plane;
mod ray;
mod rectangle;
mod scene;
mod sphere;
mod triangle;

use minifb::{Key, Window, WindowOptions};
use bsdf::BSDF;
use cgmath::prelude::*;
use intersection::Intersection;
use material::Material;
use plane::Plane;
use ray::Ray;
use rayon::prelude::*;
use rectangle::Rectangle;
use scene::Scene;
use sphere::Sphere;
use std::path::PathBuf;
use structopt::StructOpt;
use triangle::Triangle;

type Float3 = cgmath::Vector3<f64>;
type Float2 = cgmath::Vector2<f64>;

fn build_scene() -> Scene {
    Scene::new(
        // Spheres
        vec![
            Sphere::new(
                16.5,
                Float3::new(27.0, 16.5, 47.0),
                Material::new(Float3::zero(), Float3::new(1.0, 1.0, 1.0), BSDF::Mirror),
            ), //Mirror
            Sphere::new(
                16.5,
                Float3::new(73.0, 16.5, 78.0),
                Material::new(Float3::zero(), Float3::new(1.0, 1.0, 1.0), BSDF::Glass),
            ), //Glass
        ],
        // Planes
        vec![
            // Bottom
            Plane::new(
                Float3::new(0.0, 0.0, 0.0),
                Float3::new(0.0, 1.0, 0.0),
                Material::new(Float3::zero(), Float3::new(0.75, 0.75, 0.75), BSDF::Diffuse),
            ),
            // Left
            Plane::new(
                Float3::new(1.0, 0.0, 0.0),
                Float3::new(1.0, 0.0, 0.0),
                Material::new(Float3::zero(), Float3::new(0.75, 0.25, 0.25), BSDF::Diffuse),
            ),
            // Right
            Plane::new(
                Float3::new(99.0, 0.0, 0.0),
                Float3::new(-1.0, 0.0, 0.0),
                Material::new(Float3::zero(), Float3::new(0.25, 0.25, 0.75), BSDF::Diffuse),
            ),
            // Front
            Plane::new(
                Float3::new(0.0, 0.0, 0.0),
                Float3::new(0.0, 0.0, 1.0),
                Material::new(Float3::zero(), Float3::new(0.75, 0.75, 0.75), BSDF::Diffuse),
            ),
            // Back
            Plane::new(
                Float3::new(0.0, 0.0, 170.0),
                Float3::new(0.0, 0.0, -1.0),
                Material::new(Float3::zero(), Float3::zero(), BSDF::Diffuse),
            ),
            // Top
            Plane::new(
                Float3::new(0.0, 81.6, 0.0),
                Float3::new(0.0, -1.0, 0.0),
                Material::new(Float3::zero(), Float3::new(0.75, 0.75, 0.75), BSDF::Diffuse),
            ),
        ],
        // Rectangles
        vec![
            // Light
            Rectangle::new(
                Float3::new(50.0, 81.5, 50.0),
                Float3::new(0.0, -1.0, 0.0),
                Float3::new(1.0, 0.0, 0.0),
                Float3::new(0.0, 0.0, 1.0),
                33.0,
                33.0,
                Material::new(Float3::new(12.0, 12.0, 12.0), Float3::zero(), BSDF::Diffuse),
            ),
        ],
        // Triangles
        vec![
            // Triangle::new(
            //     Float3::new(20.0, 10.5, 47.0),
            //     Float3::new(20.0+32.0, 10.5, 47.0),
            //     Float3::new(20.0, 10.5+32.0, 10.0),
            //     Material::new(Float3::zero(), Float3::new(1.0, 1.0, 1.0), BSDF::Mirror),
            // ),
        ]
    )
}

fn luminance(color: Float3) -> f64 {
    0.299 * color.x + 0.587 * color.y + 0.114 * color.z
}

fn saturate(color: Float3) -> Float3 {
    Float3 {
        x: color.x.max(0.0).min(1.0),
        y: color.y.max(0.0).min(1.0),
        z: color.z.max(0.0).min(1.0),
    }
}

fn tonemap(color: Float3) -> Float3 {
    let color_linear = Float3::new(
        color.x.powf(1.0 / 2.2),
        color.y.powf(1.0 / 2.2),
        color.z.powf(1.0 / 2.2),
    );

    return saturate(color_linear);
}

fn compute_radiance(ray: Ray, scene: &Scene, depth: i32) -> Float3 {
    let intersect: Option<(Intersection, f64)> = scene.intersect(ray);

    match intersect {
        None => Float3::zero(),
        Some((intersection, t)) => {
            let position = ray.origin + ray.direction * t;
            let normal = intersection.normal;

            let mut f = intersection.material.albedo;
            if depth > 3 {
                if rand::random::<f64>() < luminance(f) && depth < 100 {
                    f = f / luminance(f);
                } else {
                    return intersection.material.emission;
                }
            }

            let irradiance: Float3 = match intersection.material.bsdf {
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
                irradiance.x * f.x + intersection.material.emission.x,
                irradiance.y * f.y + intersection.material.emission.y,
                irradiance.z * f.z + intersection.material.emission.z,
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

/// Command-line Arguments
#[derive(StructOpt, Debug)]
#[structopt(name = "smallpt", about = "A rust implementation of Kevin Beason's educational 100 lines small ray/pathtracer http://www.kevinbeason.com/smallpt/")]
struct Opt {
    /// Set sample count
    #[structopt(short = "s", long = "samples", default_value = "8")]
    samples: u32,

    /// Set final output width
    #[structopt(short = "w", long = "width", default_value = "512")]
    width: usize,

    /// Set final output height
    #[structopt(short = "h", long = "height", default_value = "512")]
    height: usize,

    /// Accumulate results over multiple frames
    #[structopt(short = "a", long = "accumulate")]
    accumulate: bool,
}

fn render(scene: &Scene, camera: &Ray, width: usize, height: usize, num_samples: u32, backbuffer: &mut [Float3]) {
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

                    // Samples per subpixel
                    for _ in 0..num_samples {

                        // jitter for AA
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

fn main() {
    // Fetch commandline arguments
    let args = Opt::from_args();
    let num_samples = args.samples.max(1);
    let width = args.width.max(1);
    let height = args.height.max(1);

    let mut backbuffer = vec![Float3::new(0.5, 0.5, 0.5); args.width * args.height];

    let scene = build_scene();

    let camera = Ray {
        origin: Float3::new(50.0, 50.0, 300.0),
        direction: Float3::new(0.0, -0.05, -1.0).normalize(),
    };

    let mut buffer: Vec<u32> = vec![0x00AAAAAA; width * height];
    let mut window = Window::new("smallpt in Rust", width, args.height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
   
    // Render
    render(&scene, &camera, width, height, num_samples, &mut backbuffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..width * height {
            let color = saturate(tonemap(backbuffer[i]));

            let r = (color.x * 255.0).round() as u32;
            let g = (color.y * 255.0).round() as u32;
            let b = (color.z * 255.0).round() as u32;

            buffer[i] = (r << 16) | (g << 8) | b;
        }

        window.update_with_buffer(&buffer).unwrap(); 
    }
}
