#![allow(unused_imports)]
#![allow(dead_code)]

extern crate smallpt;
extern crate cgmath;
extern crate minifb;
extern crate num_cpus;
extern crate rand;
extern crate rayon;

#[macro_use]
extern crate structopt;

use std::f64::consts::PI;

use smallpt::*;
use minifb::{Key, Window, WindowOptions};
use cgmath::prelude::*;
use rayon::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

// type Float3 = cgmath::Vector3<f64>;
// type Float2 = cgmath::Vector2<f64>;

fn build_scene() -> Scene {
    Scene::new(
        // Spheres
        vec![
            // Mirror
            Sphere::new(
                16.5,
                Float3::new(27.0, 16.5, 47.0),
                Material::new(Float3::zero(), Float3::new(1.0, 1.0, 1.0), BSDF::Mirror),
            ), 
            // Glass
            Sphere::new(
                16.5,
                Float3::new(73.0, 16.5, 78.0),
                Material::new(Float3::zero(), Float3::new(1.0, 1.0, 1.0), BSDF::Glass),
            ), 
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

fn main() {
    // Fetch commandline arguments
    let args = Opt::from_args();
    let num_samples = args.samples.max(1);
    let width = args.width.max(1);
    let height = args.height.max(1);
    let accumulate = args.accumulate;

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
    trace(&scene, &camera, width, height, num_samples, &mut backbuffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if accumulate { 
            trace(&scene, &camera, width, height, num_samples, &mut backbuffer); 
            
            for i in 0..width * height {
                let prev_r = ((buffer[i] >> 16)& 0xFF) as f64 / 255.0; 
                let prev_g = ((buffer[i] >> 8) & 0xFF) as f64 / 255.0; 
                let prev_b = ((buffer[i] >> 0) & 0xFF) as f64 / 255.0; 
                let prev_color = saturate(Float3::new(prev_r, prev_g, prev_b));                 

                let mut color = saturate(tonemap(backbuffer[i]));
                color = saturate(color.lerp(prev_color, 0.95)); 

                let r = (color.x * 255.0).round() as u32;
                let g = (color.y * 255.0).round() as u32;
                let b = (color.z * 255.0).round() as u32;

                buffer[i] = (r << 16) | (g << 8) | b;
            }            
        } else {
            for i in 0..width * height {
                let color = saturate(tonemap(backbuffer[i]));

                let r = (color.x * 255.0).round() as u32;
                let g = (color.y * 255.0).round() as u32;
                let b = (color.z * 255.0).round() as u32;

                buffer[i] = (r << 16) | (g << 8) | b;
            }            
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}
