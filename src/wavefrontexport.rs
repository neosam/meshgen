use std::io::Write;
use std::io::Error;
use std::result::Result;
use mesh::Mesh;
use std::collections::HashMap;

pub fn export_wavefront<W: Write>(mesh: &Mesh, w: &mut W) -> Result<(), Error> {
	try!(write!(w, "# Meshgen wavefront export\n"));
	let vertices = mesh.all_vertices();
	let mut vertex_map = HashMap::new();
	for (vertex_id, i) in vertices.iter().zip(1..) {
		vertex_map.insert(vertex_id, i);
		let vertex = mesh.get_vertex(*vertex_id).unwrap();
		try!(write!(w, "v {} {} {}\n", vertex.x, vertex.y, vertex.z));
	}
	for face in mesh.all_faces() {
		try!(write!(w, "f"));
		for vertex in face.vertices.iter() {
			let index = vertex_map.get(vertex).unwrap();
			try!(write!(w, " {}", index));
		}
		try!(write!(w, "\n"))
	}
	Ok(())
}