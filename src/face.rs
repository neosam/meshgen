use base::{Identifier, Id};

#[derive(Clone, Debug)]
pub struct Face {
	id: Identifier,
	pub vertices: Vec<Identifier>
}

impl Id for Face {
	fn get_id(&self) -> Identifier { self.id }
}

impl Face {
	pub fn new(id: Identifier, vertices: Vec<Identifier>) -> Face {
		Face {
			id: id,
			vertices: vertices
		}
	}
}