extern crate minifb;
extern crate smallpt;

use minifb::{Key, Window, WindowOptions};
use smallpt::*;
use std::time::*;

fn main() {
    let num_samples = 256;
    let width = 512;
    let height = 512;

    let mut backbuffer = vec![Float3::zero(); width * height];

    let mut scene = Scene::init();

    // Spheres
    // Mirror
    scene.add(Box::new(Sphere::new(
        16.5,
        Float3::new(27.0, 16.5, 47.0),
        Material::new(Float3::zero(), Float3::new(1.0, 1.0, 1.0), BSDF::Mirror),
    )));

    // Glass
    scene.add(Box::new(Sphere::new(
        16.5,
        Float3::new(73.0, 16.5, 78.0),
        Material::new(Float3::zero(), Float3::new(1.0, 1.0, 1.0), BSDF::Glass),
    )));

    // Planes
    // Bottom
    scene.add(Box::new(Plane::new(
        Float3::new(0.0, 0.0, 0.0),
        Float3::new(0.0, 1.0, 0.0),
        Material::new(Float3::zero(), Float3::new(0.75, 0.75, 0.75), BSDF::Diffuse),
    )));

    // Left
    scene.add(Box::new(Plane::new(
        Float3::new(1.0, 0.0, 0.0),
        Float3::new(1.0, 0.0, 0.0),
        Material::new(Float3::zero(), Float3::new(0.75, 0.25, 0.25), BSDF::Diffuse),
    )));

    // Right
    scene.add(Box::new(Plane::new(
        Float3::new(99.0, 0.0, 0.0),
        Float3::new(-1.0, 0.0, 0.0),
        Material::new(Float3::zero(), Float3::new(0.25, 0.25, 0.75), BSDF::Diffuse),
    )));

    // Front
    scene.add(Box::new(Plane::new(
        Float3::new(0.0, 0.0, 0.0),
        Float3::new(0.0, 0.0, 1.0),
        Material::new(Float3::zero(), Float3::new(0.75, 0.75, 0.75), BSDF::Diffuse),
    )));

    // Back
    scene.add(Box::new(Plane::new(
        Float3::new(0.0, 0.0, 170.0),
        Float3::new(0.0, 0.0, -1.0),
        Material::new(Float3::zero(), Float3::zero(), BSDF::Diffuse),
    )));

    // Top
    scene.add(Box::new(Plane::new(
        Float3::new(0.0, 81.6, 0.0),
        Float3::new(0.0, -1.0, 0.0),
        Material::new(Float3::zero(), Float3::new(0.75, 0.75, 0.75), BSDF::Diffuse),
    )));

    // Light (emissive rectangle)
    scene.add(Box::new(Rectangle::new(
        Float3::new(50.0, 81.5, 50.0),
        Float3::new(0.0, -1.0, 0.0),
        Float3::new(1.0, 0.0, 0.0),
        Float3::new(0.0, 0.0, 1.0),
        33.0,
        33.0,
        Material::new(Float3::new(12.0, 12.0, 12.0), Float3::zero(), BSDF::Diffuse),
    )));

    let aperture = 0.5135;
    let camera_origin = Float3::new(50.0, 50.0, 300.0);
    let camera_direction = Float3::new(0.0, -0.05, -1.0).normalize();
    let camera_right = Float3::new(width as f32 * aperture / height as f32, 0.0, 0.0);
    let camera_up = camera_right.cross(camera_direction).normalize() * aperture;

    let camera = Camera::new(
        camera_origin,
        camera_direction,
        camera_right,
        camera_up);

    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut window = Window::new("smallpt in Rust", width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut num_rays = 0;
    let start_time = SystemTime::now();

    println!(
        "Rendering {}x{} @ {} samples/pixel",
        width, height, num_samples
    );

    // Render
    trace(
        &scene,
        &camera,
        width,
        height,
        num_samples,
        &mut backbuffer,
        &mut num_rays,
    );

    // Print some metrics
    let mrays = num_rays / 1000000;
    let elapsed_seconds = start_time
        .elapsed()
        .expect("SystemTime elapsed time failed")
        .as_secs();

    println!(
        "Rendering Done: {} mrays/s, {} mrays, {} seconds",
        (mrays as f64 / elapsed_seconds as f64).round(),
        mrays,
        elapsed_seconds
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..width * height {
            let color = saturate(tonemap(backbuffer[i]));

            let r = (color.get_x() * 255.0).round() as u32;
            let g = (color.get_y() * 255.0).round() as u32;
            let b = (color.get_z() * 255.0).round() as u32;

            buffer[i] = (r << 16) | (g << 8) | b;
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}
