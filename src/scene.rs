use bsdf::BSDF;
use hit::Hit;
use ray::Ray;
use std::f64::*;
use Traceable;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<Traceable>>,
}

impl Scene {
    pub fn add(&mut self, obj: Box<Traceable>) {
        self.objects.push(obj);
    }

    pub fn init() -> Scene {
        Scene { objects: vec![] }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Hit> {
        let mut final_hit = Hit::init();

        // Intersect scene objects
        for s in 0..self.objects.len() {
            let mut current_hit = Hit::init();
            let hit = self.objects[s].intersect(&ray, &mut current_hit);

            // todo: hit min&max
            if hit == true && current_hit.t < final_hit.t && current_hit.t > 1e-6 {
                final_hit = current_hit;
            }
        }

        if final_hit.t != INFINITY {
            Some(final_hit)
        } else {
            None
        }
    }
}
