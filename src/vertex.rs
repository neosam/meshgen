use vector::Vector;
use base::{Identifier, Id};

#[derive(Clone, Debug)]
pub struct Vertex {
	id: Identifier,
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector for Vertex {
	fn get_x(&self) -> f32 { self.x }
	fn get_y(&self) -> f32 { self.y }
	fn get_z(&self) -> f32 { self.z }

	fn set_x(&mut self, x: f32) {
		self.x = x;
	}
	fn set_y(&mut self, y: f32) {
		self.y = y;
	}
	fn set_z(&mut self, z: f32) {
		self.z = z;
	}
}

impl Id for Vertex {
	fn get_id(&self) -> Identifier { self.id }
}

impl Vertex {
	pub fn new(id: Identifier, x: f32,  y: f32, z: f32) -> Vertex {
		Vertex {
			id: id, x: x, y: y, z: z
		}
	}
}