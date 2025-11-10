use std::vec;

use fatum_macros::node_impl_new;
use fatum_scene::{Node, NodeBehaviour};
use glam::Vec2;

use crate::components::Transform2D;

pub fn Node2D() -> Node {
	let mut node = Node::new();
	node.add_component(Box::new(Transform2D::default()));
	node
}

// pub struct Node2D {}

// impl Node for Node2D {
// 	fn id(&self) -> u32 {
// 		todo!()
// 	}

// 	fn name(&self) -> &str {
// 		todo!()
// 	}

// 	fn set_name(&mut self, name: &str) {
// 		todo!()
// 	}

// 	fn scene(&self) -> Option<std::sync::Arc<std::sync::Mutex<fatum_scene::SceneGraph>>> {
// 		todo!()
// 	}

// 	fn parent(&self) -> u32 {
// 		todo!()
// 	}

// 	fn children(&self) -> Vec<u32> {
// 		todo!()
// 	}

// 	fn enter_scene(&mut self, id: u32, scene: std::sync::Arc<std::sync::Mutex<fatum_scene::SceneGraph>>) {
// 		let scene = scene.lock().unwrap();
// 		scene.add_behaviour(id, Box::new(BRender));
// 	}

// 	fn exit_scene(&mut self) {
// 		todo!()
// 	}
// }

// #[derive(NodeBehaviour)]
// pub struct Node2D {
// 	pub base: Node,
// 	pub transform: Transform2D
// }

// impl Node2D {
// 	pub fn new() -> Self {
// 		let base = Node::new();

// 		Self {
// 			base,
// 			transform: Transform2D::new(Vec2::ZERO, Vec2::ZERO, Vec2::ONE)
// 		}
// 	}
// }

// impl Into<Node> for Node2D {
// 	fn into(self) -> Node {
// 		self.base
// 	}
// }
