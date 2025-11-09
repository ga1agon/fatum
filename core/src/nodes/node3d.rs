use fatum_scene::{Node, NodeBehaviour};

use crate::Transform3D;

#[derive(NodeBehaviour)]
pub struct Node3D {
	base: Node,
	transform: Transform3D
}
