use fatum_scene::{Node, NodeBehaviour};

use crate::Transform2D;

#[derive(NodeBehaviour)]
pub struct Node2D<'a> {
	base: Node<'a>,
	transform: Transform2D
}
