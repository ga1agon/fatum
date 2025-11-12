use std::rc::Rc;

use fatum_scene::{Node, NodeComponent, NodeId, SharedSceneGraph};

use crate::components::{self, Transform3D};

pub struct Model3D {}

impl Model3D {
	pub fn new(model: Rc<Box<fatum_graphics::Model>>) -> Node {
		let model = Box::new(components::Model::new(model));
		let t3d = Box::new(Transform3D::default());

		let mut node = Node::new();
		node.add_component(model);
		node.add_component(t3d);
		node
	}
}
