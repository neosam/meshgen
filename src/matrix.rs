use vector::Vector;
use vector::VectorImpl;

#[derive(Clone,Debug,PartialEq)]
pub struct Matrix {
	pub values: [f32;16]
}

#[derive(Clone,Debug,PartialEq)]
pub struct Vector4 {
	pub values: [f32;4]
}

impl Matrix {
	pub fn identity() -> Matrix {
		Matrix {
			values: [
				1f32, 0f32, 0f32, 0f32,
				0f32, 1f32, 0f32, 0f32,
				0f32, 0f32, 1f32, 0f32,
				0f32, 0f32, 0f32, 1f32
			]
		}
	}
	pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
		Matrix {
			values: [
				1f32, 0f32, 0f32, 0f32,
				0f32, 1f32, 0f32, 0f32,
				0f32, 0f32, 1f32, 0f32,
				x,    y,    z,    1f32
			]
		}
	}
	pub fn scale(x: f32, y: f32, z: f32) -> Matrix {
		Matrix {
			values: [
				x,    0f32, 0f32, 0f32,
				0f32, y,    0f32, 0f32,
				0f32, 0f32, z,    0f32,
				0f32, 0f32, 0f32, 1f32
			]
		}
	}
	pub fn rot(a: f32, x_vec: f32, y_vec: f32, z_vec: f32) -> Matrix {
		let length = (x_vec * x_vec + y_vec * y_vec + z_vec * z_vec).sqrt();
		let x = x_vec / length;
		let y = y_vec / length;
		let z = z_vec / length;
		let xx = x * x;
		let yy = y * y;
		let zz = z * z;
		let xy = x * y;
		let xz = x * z;
		let yz = y * z;
		let asin = a.sin();
		let acos = a.cos();
		Matrix {
			values: [
				xx + (1f32 - xx) * acos, xy + (1f32 - xy) * asin + z * asin, xz + (1f32 - xz) * asin - y * asin, 0f32,
				xy + (1f32 - xy) * asin - z * asin, yy + (1f32 - yy) * acos, yz + (1f32 - yz) * asin + x * asin, 0f32,
				xz + (1f32 - xz) * asin + y * asin, yz + (1f32 - yz) * asin - x * asin, zz + (1f32 - zz) * acos, 0f32,
				0f32, 0f32, 0f32, 1f32
			]
		}
	}
	pub fn mult_vector4(&self, v: &Vector4) -> Vector4 {
		let mut res = Vector4 {
			values: [0f32; 4]
		};
		for i in 0..4 {
			for j in 0..4 {	
				res.values[i] += v.values[j] * self.values[j * 4 + i];
			}
		}
		res
	}
	pub fn mult_vector<V: Vector>(&self, v: &V) -> VectorImpl {
		let attr = Vector4 {
			values: [v.get_x(), v.get_y(), v.get_z(), 1f32]
		};
		let res = self.mult_vector4(&attr);
		VectorImpl::new(res.values[0], res.values[1], res.values[2])
	}
	pub fn get_vec(&self, m: usize) -> Vector4 {
		let mut res = Vector4 {
			values: [0f32; 4]
		};
		for i in 0..4 {
			res.values[i] = self.values[i + m * 4]
		}
		res
	}
	pub fn mult_matrix(&self, m: &Matrix) -> Matrix {
		let v1 = self.mult_vector4(&m.get_vec(0));
		let v2 = self.mult_vector4(&m.get_vec(1));
		let v3 = self.mult_vector4(&m.get_vec(2));
		let v4 = self.mult_vector4(&m.get_vec(3));
		let mut res = Matrix { values: [0f32; 16] };
		for i in 0..4 {
			res.values[i] = v1.values[i];
			res.values[i + 4] = v2.values[i];
			res.values[i + 8] = v3.values[i];
			res.values[i + 12] = v4.values[i];
		}
		res
	}
}


#[test]
pub fn rot_matrix_test() {
	let x_rot = Matrix::rot(0f32, 1f32, 0f32, 0f32);
	let y_rot = Matrix::rot(0f32, 0f32, 1f32, 0f32);
	let z_rot = Matrix::rot(0f32, 0f32, 0f32, 1f32);
	let identity = Matrix::identity();
	assert_eq!(identity, x_rot);
	assert_eq!(identity, y_rot);
	assert_eq!(identity, z_rot);
}
