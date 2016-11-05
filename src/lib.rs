//! Procedural 3d mesh generation
//!
//! This library tries to provide tools to create mesh tools
//! the procedural way.

pub mod base;
pub mod matrix;
pub mod vector;
pub mod vertex;
pub mod face;
pub mod mesh;
pub mod wavefrontexport;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
