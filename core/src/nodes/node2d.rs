use fatum_scene::{Node, NodeBehaviour};

use crate::Transform2D;

#[derive(NodeBehaviour)]
pub struct Node2D {
	base: Node,
	transform: Transform2D
}
