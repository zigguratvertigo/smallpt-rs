use material::Material;
use bvh::*;
use plane::Plane;
use ray::Ray;
use PrimitiveType;
use Traceable;

#[derive(Copy, Clone)]
pub struct Rectangle {
	pub position: Vector3,
	pub normal: Vector3,
	pub left: Vector3,
	pub up: Vector3,
	pub width: f32,
	pub height: f32,
	pub material: Material,
}

impl Rectangle {
	// Spawn a new rectangle
	pub fn new(
		position: Vector3,
		normal: Vector3,
		left: Vector3,
		up: Vector3,
		width: f32,
		height: f32,
		material: Material,
	) -> Rectangle {
		Rectangle {
			position: position,
			normal: normal,
			left: left,
			up: up,
			width: width,
			height: height,
			material: material,
		}
	}
}

impl Traceable for Rectangle {
	// Ray-Rectangle Intersection
	fn intersect(&self, r: &Ray, result: &mut ::Hit) -> bool {
		let rectangle_plane = Plane::new(self.position, self.normal, self.material);
		let hit = rectangle_plane.intersect(r, result);

		if hit == true {
			let p = r.origin + r.direction * result.t;
			let v = p - self.position;

			let half_width = self.width * 0.5;
			let half_height = self.height * 0.5;

			// Project in 2D plane and clamp inside the rectangle
			if v.dot(self.left).abs() <= half_width && v.dot(self.up).abs() <= half_height {
				result.p = p;
				result.n = if self.normal.dot(r.direction) < 0.0 {
					self.normal
				} else {
					-self.normal
				};
				result.material = self.material;
				return true;
			}
		}

		return false;
	}

	fn get_primitive_type(&self) -> PrimitiveType {
		PrimitiveType::Rectangle
	}
}
