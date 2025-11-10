use fatum_scene::{Node, NodeBehaviour};

use crate::components::Transform3D;

pub fn Node3D() -> Node {
	let mut node = Node::new();
	node.add_component(Box::new(Transform3D::default()));
	node
}

// #[derive(NodeBehaviour)]
// pub struct Node3D {
// 	base: Node,
// 	transform: Transform3D
// }
