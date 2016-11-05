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

	fn sub_from<V: Vector>(&mut self, v: &V) {
		let x = self.get_x();
		let y = self.get_y();
		let z = self.get_z();		
		self.set_x(x - v.get_x());
		self.set_y(y - v.get_y());
		self.set_z(z - v.get_z());
	}

	fn mult_scalar_to(&mut self, factor: f32) {
		let x = self.get_x();
		let y = self.get_y();
		let z = self.get_z();
		self.set_x(x * factor);
		self.set_y(y * factor);
		self.set_z(z * factor);
	}

	fn cross_prod_to<V: Vector>(&mut self, v: &V) {
		let x = self.get_y() * v.get_z() - v.get_y() * self.get_z();
		let y = self.get_z() * v.get_x() - v.get_z() * self.get_x();
		let z = self.get_x() * v.get_y() - v.get_x() * self.get_y();
		self.set_x(x);
		self.set_y(y);
		self.set_z(z);
	}

	fn length(&self) -> f32 {
		let sum = self.get_x() * self.get_x()
					+ self.get_y() * self.get_y()
					+ self.get_z() * self.get_z();
		sum.sqrt()
	}

	fn normalize(&mut self) {
		let length = self.length();
		let x = self.get_x();
		let y = self.get_y();
		let z = self.get_z();
		self.set_x(x / length);
		self.set_y(y / length);
		self.set_z(z / length);
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
