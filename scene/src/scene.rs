use std::{rc::Rc, sync::{Arc, Mutex}};

use crate::{Node, NodeBehaviour};

pub struct SceneTree<'a> {
	root_node: Node<'a>
}

impl<'a> SceneTree<'a> {
	pub fn new() -> Arc<Mutex<Self>> {
		let s = Arc::new(Mutex::new(Self {
			root_node: Node::new(vec![])
		}));

		s.lock().unwrap().root_node.enter_scene(s.clone(), None);
		s
	}
}
