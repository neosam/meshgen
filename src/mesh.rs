use std::rc::Rc;
use face::Face;
use std::collections::BTreeMap;
use vertex::Vertex;
use base::*;

pub struct Mesh {
	id_counter: Identifier,
	faces: BTreeMap<Identifier, Rc<Face>>,
	vertex_face_map: BTreeMap<Identifier, Vec<Identifier>>
}

impl Mesh {
	pub fn gen_id(&mut self) -> Identifier {
		let res = self.id_counter;
		self.id_counter += 1;
		res
	}

	pub fn gen_vertex(&mut self, x: f32, y: f32, z: f32) -> Rc<Vertex> {
		let new_id = self.gen_id();
		self.vertex_face_map.insert(new_id, Vec::new());
		Rc::new(Vertex::new(new_id, x, y, z))
	}

	pub fn gen_face(&mut self, vertices: &[Rc<Vertex>]) -> Rc<Face> {
		let face_vertices = vertices.to_vec();
		let face = Rc::new(Face::new(self.gen_id(), face_vertices));
		self.faces.insert(face.get_id(), face.clone());
		for vertex in vertices {
			let vec = self.vertex_face_map.get_mut(&vertex.get_id()).unwrap();

			vec.push(face.get_id());
		}
		face
	}

	pub fn all_faces(&self) -> Vec<Rc<Face>> {
		self.faces.values().map(|x| x.clone()).collect()
	}
}

impl Default for Mesh {
	fn default() -> Mesh {
		Mesh {
			id_counter: 0,
			faces: BTreeMap::new(),
			vertex_face_map: BTreeMap::new()
		}
	}
}

#[test]
fn test() {
	let mut mesh = Mesh::default();
	assert_eq!(0, mesh.all_faces().len());

	let vertices = vec![
		mesh.gen_vertex(-1f32, -1f32, 0f32),
		mesh.gen_vertex( 1f32, -1f32, 0f32),
		mesh.gen_vertex( 1f32,  1f32, 0f32),
		mesh.gen_vertex(-1f32,  1f32, 0f32)
	];
	/*let f = */mesh.gen_face(&vertices);

	assert_eq!(1, mesh.all_faces().len());
}