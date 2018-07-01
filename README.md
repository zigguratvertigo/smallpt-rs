[![license](https://img.shields.io/github/license/mashape/apistatus.svg)]()
[![Crates.io](https://img.shields.io/crates/v/smallpt.svg)](https://crates.io/crates/smallpt)
[![Build Status](https://travis-ci.org/zigguratvertigo/smallpt-rs.svg?branch=master)](https://travis-ci.org/zigguratvertigo/smallpt-rs)

A [Rust](https://www.rust-lang.org/) implementation of a small ray/pathtracer.

Inspired by [Kevin Beason's educational 99-line raytracer/pathtracer](http://www.kevinbeason.com/smallpt/).

![alt text](https://github.com/zigguratvertigo/smallpt-rs/blob/master/smallpt.png)

Supports:
- Ray-to-Sphere
- Ray-to-Plane
- Ray-to-Rectangle
- Ray-to-Triangle (slow, no acceleration yet. Soon)

Usage
-----

```toml
# Cargo.toml
[dependencies]
smallpt = "0.1.12"
```

Example
-------
```rust
extern crate smallpt;
use smallpt::*;

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

let camera = Camera::new(camera_origin, camera_direction, camera_right, camera_up);

// Render
let num_samples = 16;
let width = 512;
let height = 512;
let mut num_rays = 0;
let mut backbuffer = vec![Float3::zero(); width * height];   
trace(&scene, &camera, width, height, num_samples, &mut backbuffer, &mut num_rays);
```

Status
------
Code is still quite in flux, being refined on a weekly basis. More simplification and changes coming soon.
