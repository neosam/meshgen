use std::rc::Rc;
use vertex::Vertex;
use base::{Identifier, Id};

pub struct Face {
	id: Identifier,
	pub vertices: Vec<Rc<Vertex>>
}

impl Id for Face {
	fn get_id(&self) -> Identifier { self.id }
}

impl Face {
	pub fn new(id: Identifier, vertices: Vec<Rc<Vertex>>) -> Face {
		Face {
			id: id,
			vertices: vertices
		}
	}
}