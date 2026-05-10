use crate::hit::Hit;
use crate::material::Material;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::{PrimitiveType, Traceable, Vec3};

#[derive(Copy, Clone)]
pub struct Rectangle {
	pub position: Vec3,
	pub normal: Vec3,
	pub left: Vec3,
	pub up: Vec3,
	pub width: f32,
	pub height: f32,
	pub material: Material,
}

impl Rectangle {
	pub fn new(
		position: Vec3,
		normal: Vec3,
		left: Vec3,
		up: Vec3,
		width: f32,
		height: f32,
		material: Material,
	) -> Rectangle {
		Rectangle {
			position,
			normal,
			left,
			up,
			width,
			height,
			material,
		}
	}
}

impl Traceable for Rectangle {
	fn intersect(&self, r: &Ray, result: &mut Hit) -> bool {
		let rectangle_plane = Plane::new(self.position, self.normal, self.material);
		let hit = rectangle_plane.intersect(r, result);

		if hit {
			let p = r.origin + r.direction * result.t;
			let v = p - self.position;

			let half_width = self.width * 0.5;
			let half_height = self.height * 0.5;

			if v.dot(&self.left).abs() <= half_width && v.dot(&self.up).abs() <= half_height {
				result.p = p;
				result.n = if self.normal.dot(&r.direction) < 0.0 {
					self.normal
				} else {
					-self.normal
				};
				result.material = self.material;
				return true;
			}
		}

		false
	}

	fn get_primitive_type(&self) -> PrimitiveType {
		PrimitiveType::Rectangle
	}
}
