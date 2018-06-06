use Traceable;
use bsdf::BSDF;
use hit::Hit;
use material::Material;
use plane::Plane;
use ray::Ray;
use rectangle::Rectangle;
use sphere::Sphere;
use triangle::Triangle;
use std;

use cgmath::prelude::*;
extern crate cgmath;
type Float3 = cgmath::Vector3<f64>;

#[derive(Default)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
    pub rectangles: Vec<Rectangle>,
    pub triangles: Vec<Triangle>,
}

impl Scene {

    // refactor: single Traceable type

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_plane(&mut self, plane: Plane) {
        self.planes.push(plane);
    }

    pub fn add_rectangle(&mut self, rectangle: Rectangle) {
        self.rectangles.push(rectangle);
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    pub fn init() -> Scene {
        Scene::new(vec![], vec![], vec![], vec![] )
    }

    pub fn new(spheres: Vec<Sphere>, planes: Vec<Plane>, rectangles: Vec<Rectangle>, triangles: Vec<Triangle>) -> Scene {
        Scene {
            spheres: spheres,
            planes: planes,
            rectangles: rectangles,
            triangles: triangles,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<(Hit, f64)> {
        let mut result = (0, std::f64::INFINITY);
        let mut intersection_position = Float3::zero();
        let mut intersection_normal = Float3::zero();
        let mut intersection_material = Material::black();

        // Intersect Spheres
        for s in 0..self.spheres.len() {
            let sphere = &self.spheres[s];
            let hit_t = sphere.intersect(&ray);
            let (_, prev_t) = result;

            if hit_t < prev_t && hit_t > 1e-6 {
                result = (s, hit_t);

                intersection_position = self.spheres[s].position;

                let position = ray.origin + ray.direction * hit_t;
                intersection_normal = (position - intersection_position).normalize();
                intersection_normal = if Float3::dot(intersection_normal, ray.direction) < 0.0 {
                    intersection_normal
                } else {
                    intersection_normal * -1.0
                };

                intersection_material = self.spheres[s].material;
            }
        }

        // Intersect Planes
        for s in 0..self.planes.len() {
            let plane = &self.planes[s];
            let hit_t = plane.intersect(&ray);
            let (_, prev_t) = result;

            if hit_t < prev_t && hit_t > 1e-6 {
                result = (s, hit_t);

                let position = ray.origin + ray.direction * hit_t;
                intersection_position =  position;

                intersection_normal = if Float3::dot(plane.normal, ray.direction) < 0.0 {
                    plane.normal
                } else {
                    plane.normal * -1.0
                };

                intersection_material = plane.material;
            }
        }

        // Intersect Rectangles
        for s in 0..self.rectangles.len() {
            let rectangle = &self.rectangles[s];
            let hit_t = rectangle.intersect(&ray);
            let (_, prev_t) = result;

            if hit_t < prev_t && hit_t > 1e-6 {
                result = (s, hit_t);

                let position = ray.origin + ray.direction * hit_t;
                intersection_position =  position;

                intersection_normal = if Float3::dot(rectangle.normal, ray.direction) < 0.0 {
                    rectangle.normal
                } else {
                    rectangle.normal * -1.0
                };

                intersection_material = rectangle.material;
            }
        }

        // Intersect Triangles
        for s in 0..self.triangles.len() {
            let triangle = &self.triangles[s];
            let hit_t = triangle.intersect(&ray);
            let (_, prev_t) = result;

            if hit_t < prev_t && hit_t > 1e-6 {
                result = (s, hit_t);

                let position = ray.origin + ray.direction * hit_t;
                intersection_position =  position;

                intersection_normal = if Float3::dot(triangle.normal, ray.direction) < 0.0 {
                    triangle.normal
                } else {
                    triangle.normal * -1.0
                };

                intersection_material = triangle.material;
            }
        }        

        let (_, hit_t) = result;
        if hit_t != std::f64::INFINITY {
            Some((
                Hit::new(
                    intersection_position,
                    intersection_normal,
                    intersection_material,
                ),
                hit_t,
            ))
        } else {
            None
        }
    }
}
