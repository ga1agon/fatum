use std::vec;

use fatum_macros::node_impl_new;
use fatum_scene::{Node, NodeBehaviour};
use glam::Vec2;

use crate::Transform2D;

#[derive(NodeBehaviour)]
pub struct Node2D {
	pub base: Node,
	pub transform: Transform2D
}

impl Node2D {
	pub fn new() -> Self {
		let base = Node::new();

		Self {
			base,
			transform: Transform2D::new(Vec2::ZERO, Vec2::ZERO, Vec2::ONE)
		}
	}
}

impl Into<Node> for Node2D {
	fn into(self) -> Node {
		self.base
	}
}
