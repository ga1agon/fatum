use fatum_scene::Node;

use crate::Transform3D;

pub struct Node3D<'a> {
	base: Node<'a>,
	transform: Transform3D
}
