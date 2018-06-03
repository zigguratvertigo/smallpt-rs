use cgmath::prelude::*;
extern crate cgmath;
type Float3 = cgmath::Vector3<f64>;

use std::f64::consts::PI;

extern crate rand;

use bsdf::BSDF;
use cgmath::prelude::*;
use intersection::Intersection;
use material::Material;
use plane::Plane;
use ray::Ray;
use rectangle::Rectangle;
use scene::Scene;
use sphere::Sphere;

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

            let irradiance = match intersection.material.bsdf {
                // Diffuse Reflection
                BSDF::Diffuse => {
                    // Sample cosine distribution and transform into world tangent space
                    let r1 = 2.0 * PI * rand::random::<f64>();
                    let r2 = rand::random::<f64>();
                    let r2s = r2.sqrt();
                    let wup = if normal.x.abs() > 0.1 {
                        Float3::new(0.0, 1.0, 0.0)
                    } else {
                        Float3::new(1.0, 0.0, 0.0)
                    };

                    let tangent = normal.cross(wup).normalize();
                    let bitangent = normal.cross(tangent).normalize();
                    let next_direction = tangent * r1.cos() * r2s + bitangent * r1.sin() * r2s
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

                    // Compute fresnel.
                    let into = Float3::dot(normal, normal) > 0.0;
                    let nc = 1.0;
                    let nt = 1.5;
                    let nnt = if into { nc / nt } else { nt / nc };
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

                        let reflectance =
                            base_reflectance + (1.0 - base_reflectance) * c * c * c * c * c;
                        let transmittance = 1.0 - reflectance;
                        let rr_propability = 0.25 + 0.5 * reflectance;
                        let reflectance_propability = reflectance / rr_propability;
                        let transmittance_propability = transmittance / (1.0 - rr_propability);

                        if depth > 1 {
                            // Russian roulette between reflectance and transmittance
                            if rand::random::<f64>() < rr_propability {
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

            return Float3::new(
                irradiance.x * f.x + intersection.material.emission.x,
                irradiance.y * f.y + intersection.material.emission.y,
                irradiance.z * f.z + intersection.material.emission.z,
            );
        }
    }
}
