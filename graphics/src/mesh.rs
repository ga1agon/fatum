use crate::{Material, Vertex};

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>,
	pub material: Material
}
