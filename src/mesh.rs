use face::Face;
use std::collections::BTreeMap;
use vertex::Vertex;
use vector::Vector;
use base::*;

pub struct Mesh {
	id_counter: Identifier,
	faces: BTreeMap<Identifier, Face>,
	vertices: BTreeMap<Identifier, Vertex>,
	vertex_face_map: BTreeMap<Identifier, Vec<Identifier>>
}

pub struct ExtrudeResult {
	pub top_face: Identifier,
	pub side_faces: Vec<Identifier>
}

impl Mesh {
	pub fn gen_id(&mut self) -> Identifier {
		let res = self.id_counter;
		self.id_counter += 1;
		res
	}

	pub fn gen_vertex(&mut self, x: f32, y: f32, z: f32) -> Identifier {
		let new_id = self.gen_id();
		let vertex = Vertex::new(new_id, x, y, z);
		self.vertex_face_map.insert(new_id, Vec::new());
		self.vertices.insert(new_id, vertex);
		new_id
	}

	pub fn gen_face(&mut self, vertices: &[Identifier]) -> Identifier {
		let new_id = self.gen_id();
		let face_vertices = vertices.to_vec();
		let face = Face::new(new_id, face_vertices);
		for vertex in vertices {
			let vec = self.vertex_face_map.get_mut(&vertex).unwrap();
			vec.push(face.get_id());
		}
		self.faces.insert(new_id, face);
		new_id
	}

	pub fn vertices_of_face(&self, id: Identifier) -> Option<Vec<Vertex>> {
		self.faces.get(&id).and_then(|face| 
			Some(face.vertices.iter().map(
				|x| self.get_vertex(*x).unwrap().clone()).collect()))
	}

	pub fn get_vertex(&self, id: Identifier) -> Option<&Vertex> {
		self.vertices.get(&id)
	}
	pub fn get_vertex_clone(&self, id: Identifier) -> Option<Vertex> {
		self.get_vertex(id).and_then(|v| Some(v.clone()))
	}
	pub fn get_face(&self, id: Identifier) -> Option<&Face> {
		self.faces.get(&id)
	}
	pub fn get_face_clone(&self, id: Identifier) -> Option<Face> {
		self.get_face(id).and_then(|f| Some(f.clone()))
	}
	pub fn update_vertex(&mut self, vertex: &Vertex) {
		self.vertices.insert(vertex.get_id(), vertex.clone());
	}
	pub fn delete_face(&mut self, id: Identifier) {
		self.faces.remove(&id);
	}
	pub fn move_vertex<V: Vector>(&mut self, id: Identifier, v: &V) {
		let mut vertex = {
			let vertex_option = self.get_vertex(id);
			if vertex_option.is_none() {
				return ()
			}
			vertex_option.unwrap().clone()
		};
		vertex.add_to(v);
		self.update_vertex(&vertex);				
	}
	pub fn move_face<V: Vector>(&mut self, id: Identifier, v: &V) {
		let vertices = {
			let face_option = self.get_face(id);
			if face_option.is_none() {
				return
			}
			face_option.unwrap().vertices.clone()
		};
		for vertex_id in vertices {
			self.move_vertex(vertex_id, v);
		}
	}

	pub fn extrude_face<V: Vector>(&mut self, face_id: Identifier, v: &V) -> Option<ExtrudeResult> {
		/* Get the vertices */
		self.get_face_clone(face_id).and_then(|face| {
			let mut side_faces = Vec::new();
			//let face_vertices = face.vertices;

			/* Create the new vertices of the extruded face and move them */
			let new_vertices = self.duplicate_vertices(&face.vertices);
			self.transform_vertices(new_vertices.as_slice(), | mut vertex | {
				vertex.add_to(v);
				vertex
			});

			/* Building the side faces */
			let mut last_face_vertex = *face.vertices.last().unwrap();
			let mut last_new_vertex = *new_vertices.last().unwrap();
			for (face_vertex, new_vertex) in face.vertices.iter().zip(new_vertices.iter()) {
				let side_face_vertices = [last_face_vertex, *face_vertex, *new_vertex, last_new_vertex];
				let side_face = self.gen_face(&side_face_vertices);
				side_faces.push(side_face);
				last_face_vertex = *face_vertex;
				last_new_vertex = *new_vertex;
			}

			/* Delete the original inner face */
			self.delete_face(face_id);

			/* Generate the new face */
			let top_face = self.gen_face(new_vertices.as_slice());

			Some(ExtrudeResult {
				top_face: top_face,
				side_faces: side_faces
			})
		})
	}

	pub fn duplicate_vertices(&mut self, vertices: &[Identifier]) -> Vec<Identifier> {
		vertices.iter().map(|id| self.duplicate_vertex(*id)).collect()
	}

	pub fn duplicate_vertex(&mut self, id: Identifier) -> Identifier {
		self.get_vertex_clone(id).and_then(|vertex| {
			Some(self.gen_vertex(vertex.x, vertex.y, vertex.z))
		}).unwrap_or(-1)
	}

	pub fn all_faces(&self) -> Vec<Face> {
		self.faces.values().map(|x| x.clone()).collect()
	}

	pub fn transform_vertices<F>(&mut self, ids: &[Identifier], f: F) 
			where F: Fn(Vertex) -> Vertex {
		for id in ids {
			let vertex_in = {
				let vertex_in_option = self.get_vertex(*id);
				if vertex_in_option.is_none() {
					continue;
				}
				vertex_in_option.unwrap().clone()
			};
			let vertex_out = f(vertex_in);
			self.update_vertex(&vertex_out);
		}
	}
}

impl Default for Mesh {
	fn default() -> Mesh {
		Mesh {
			id_counter: 0,
			faces: BTreeMap::new(),
			vertices: BTreeMap::new(),
			vertex_face_map: BTreeMap::new()
		}
	}
}

#[test]
fn test() {
	use vector::VectorImpl;

	let mut mesh = Mesh::default();
	assert_eq!(0, mesh.all_faces().len());
	assert_eq!(true, mesh.get_vertex(1).is_none());

	let v1 = mesh.gen_vertex(-1f32, -1f32, 0f32);
	assert_eq!(-1f32, mesh.get_vertex(v1).unwrap().x);
	assert_eq!(-1f32, mesh.get_vertex(v1).unwrap().y);
	assert_eq!(0f32, mesh.get_vertex(v1).unwrap().z);
	let v2 = mesh.gen_vertex( 1f32, -1f32, 0f32);
	let v3 = mesh.gen_vertex( 1f32,  1f32, 0f32);
	let v4 = mesh.gen_vertex(-1f32,  1f32, 0f32);
	let ids: Vec<Identifier> = vec![v1, v2, v3, v4];
	let f = mesh.gen_face(ids.as_slice());
	assert_eq!(1, mesh.all_faces().len());
	assert_eq!(ids, mesh.get_face(f).unwrap().vertices);

	let vertices = mesh.vertices_of_face(f).unwrap();
	assert_eq!(-1f32, vertices[0].x);
	assert_eq!(-1f32, vertices[1].y);
	assert_eq!(0f32, vertices[2].z);

	assert_eq!(-1f32, mesh.get_vertex(v1).unwrap().x);
	mesh.move_vertex(v1, &VectorImpl::new(1f32, 0f32, 0f32));
	assert_eq!(0f32, mesh.get_vertex(v1).unwrap().x);

	let extrude_result = mesh.extrude_face(f, &VectorImpl::new(0f32, 0f32, 1f32)).unwrap();
	assert_eq!(4, extrude_result.side_faces.len());
	assert_eq!(5, mesh.all_faces().len());
	let top_face = mesh.get_face(extrude_result.top_face).unwrap();
	assert_eq!(1f32, mesh.get_vertex(top_face.vertices[0]).unwrap().z);
	assert_eq!(1f32, mesh.get_vertex(top_face.vertices[1]).unwrap().z);
	assert_eq!(1f32, mesh.get_vertex(top_face.vertices[2]).unwrap().z);
	assert_eq!(1f32, mesh.get_vertex(top_face.vertices[3]).unwrap().z);
}

