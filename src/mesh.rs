use face::Face;
use std::collections::HashMap;
use std::collections::HashSet;
use vertex::Vertex;
use vector::Vector;
use base::*;


pub struct Mesh {
	id_counter: Identifier,
	faces: HashMap<Identifier, Face>,
	vertices: HashMap<Identifier, Vertex>,
	vertex_face_map: HashMap<Identifier, Vec<Identifier>>
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
	pub fn get_vertex_mut(&mut self, id: Identifier) -> Option<&mut Vertex> {
		self.vertices.get_mut(&id)
	}
	pub fn get_vertex_clone(&self, id: Identifier) -> Option<Vertex> {
		self.get_vertex(id).and_then(|v| Some(v.clone()))
	}
	pub fn get_face(&self, id: Identifier) -> Option<&Face> {
		self.faces.get(&id)
	}
	pub fn get_face_mut(&mut self, id: Identifier) -> Option<&mut Face> {
		self.faces.get_mut(&id)
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

	pub fn face_normal(&self, face_id: Identifier) -> Vertex {
		let face = self.get_face(face_id).unwrap().clone();
		let v1 = self.get_vertex(face.vertices[0]).unwrap();
		let v2 = self.get_vertex(face.vertices[1]).unwrap();
		let v3 = self.get_vertex(face.vertices[2]).unwrap();
		let mut edge1 = v2.clone();
		edge1.sub_from(v1);
		let mut edge2 = v3.clone();
		edge2.sub_from(v2);
		edge1.cross_prod_to(&edge2);
		edge1.normalize();
		edge1
	}

	pub fn extrude_face_normal(&mut self, face_id: Identifier, length: f32) -> Option<ExtrudeResult> {
		let mut face_normal = self.face_normal(face_id);
		face_normal.mult_scalar_to(length);
		self.extrude_face(face_id, &face_normal)
	}

	pub fn extrude_face<V: Vector>(&mut self, face_id: Identifier, v: &V) -> Option<ExtrudeResult> {
		/* Get the vertices */
		self.get_face_clone(face_id).and_then(|face| {
			let mut side_faces = Vec::new();
			//let face_vertices = face.vertices;

			/* Create the new vertices of the extruded face and move them */
			let new_vertices = self.duplicate_vertices(&face.vertices);
			self.transform_vertices(new_vertices.as_slice(), | vertex | {
				vertex.add_to(v);
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

	pub fn all_vertices(&self) -> Vec<Identifier> {
		let mut vertices: HashSet<Identifier> = HashSet::new();
		for face in self.all_faces().iter() {
			for identifier in face.vertices.iter() {
				vertices.insert(*identifier);
			}
		}
		vertices.iter().map(|x| x.clone()).collect()
	}

	pub fn transform_vertices<F>(&mut self, ids: &[Identifier], f: F) 
			where F: Fn(&mut Vertex) {
		for id in ids {
			self.get_vertex_mut(*id).and_then(|vertex| {
				f(vertex);
				Some(())
			});
		}
	}

	pub fn transform_vertices_to_point<F, V: Vector>(&mut self, v: &V, ids: &[Identifier], f: F) 
			where F: Fn(&mut Vertex) {
		for id in ids {
			self.get_vertex_mut(*id).and_then(|vertex| {
				vertex.sub_from(v);
				f(vertex);
				vertex.add_to(v);
				Some(())
			});
		}
	}
}

impl Default for Mesh {
	fn default() -> Mesh {
		Mesh {
			id_counter: 0,
			faces: HashMap::new(),
			vertices: HashMap::new(),
			vertex_face_map: HashMap::new()
		}
	}
}

#[test]
fn test() {
	use vector::VectorImpl;
	use wavefrontexport::export_wavefront;
	use std::fs::File;

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
	{
		let top_face = mesh.get_face(extrude_result.top_face).unwrap();
		assert_eq!(1f32, mesh.get_vertex(top_face.vertices[0]).unwrap().z);
		assert_eq!(1f32, mesh.get_vertex(top_face.vertices[1]).unwrap().z);
		assert_eq!(1f32, mesh.get_vertex(top_face.vertices[2]).unwrap().z);
		assert_eq!(1f32, mesh.get_vertex(top_face.vertices[3]).unwrap().z);
	}
	let extrude_result2 = mesh.extrude_face(
		extrude_result.top_face, 
		&VectorImpl::new(1f32, 0f32, 3f32)).unwrap();
	mesh.extrude_face_normal(
		extrude_result.side_faces[0], 3f32).unwrap();
	assert_eq!(4, extrude_result.side_faces.len());
	assert_eq!(13, mesh.all_faces().len());

	mesh.gen_face(ids.as_slice());

	let mut wavefront = File::create("export.obj").unwrap();
	export_wavefront(&mesh, &mut wavefront);
}

