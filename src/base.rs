pub type Identifier = i32;

pub trait Id {
	fn get_id(&self) -> Identifier;
}