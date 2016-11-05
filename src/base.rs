//! Contains helper 

/// Internal mesh identifier 
pub type Identifier = i32;

/// A type which return a mesh identifier
pub trait Id {
	fn get_id(&self) -> Identifier;
}