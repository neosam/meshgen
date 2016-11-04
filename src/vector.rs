pub trait Vector {
	fn get_x(&self) -> f32;
	fn get_y(&self) -> f32;
	fn get_z(&self) -> f32;

	fn set_x(&mut self, x: f32);
	fn set_y(&mut self, y: f32);
	fn set_z(&mut self, z: f32);

	fn add_to<V: Vector>(&mut self, v: &V) {
		let x = self.get_x();
		let y = self.get_y();
		let z = self.get_z();
		self.set_x(x + v.get_x());
		self.set_y(y + v.get_y());
		self.set_z(z + v.get_z());
	}
}

#[derive(Debug)]
pub struct VectorImpl {
    x: f32,
    y: f32,
    z: f32
}

impl Vector for VectorImpl {
	fn get_x(&self) -> f32 { self.x }
	fn get_y(&self) -> f32 { self.y }
	fn get_z(&self) -> f32 { self.z }
	fn set_x(&mut self, x: f32) { self.x = x }
	fn set_y(&mut self, y: f32) { self.y = y }
	fn set_z(&mut self, z: f32) { self.z = z }
}

impl VectorImpl {
	pub fn new(x: f32, y: f32, z: f32) -> VectorImpl {
		VectorImpl { x: x, y: y, z: z }
	}
}
