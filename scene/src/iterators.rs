use std::sync::RwLockWriteGuard;

use crate::{NodeId, SceneGraph, SharedSceneGraph};

pub struct SceneDfsIterator {
	scene: SharedSceneGraph,
	stack: Vec<NodeId>
}

impl SceneDfsIterator {
	pub fn new(scene: SharedSceneGraph, root: NodeId) -> Self {
		Self {
			scene,
			stack: vec![root]
		}
	}
}

impl Iterator for SceneDfsIterator {
	type Item = NodeId;

	fn next(&mut self) -> Option<Self::Item> {
		if let Ok(scene) = self.scene.try_read() {
			let node = self.stack.pop()?;
			let mut children = self.scene.read().unwrap().children(node);
			children.reverse();

			self.stack.extend(children);

			return Some(node);
		}

		log::warn!("Could not get a read lock on the scene");
		None
	}
}

pub struct ScenePostDfsIterator {
	scene: SharedSceneGraph,
	stack: Vec<(NodeId, bool)>
}

impl ScenePostDfsIterator {
	pub fn new(scene: SharedSceneGraph, root: NodeId) -> Self {
		Self {
			scene,
			stack: vec![(root, false)]
		}
	}
}

impl Iterator for ScenePostDfsIterator {
	type Item = NodeId;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some((node, visited)) = self.stack.pop() {
			if visited {
				return Some(node);
			}

			self.stack.push((node, true));

			let children = self.scene.read().unwrap().children(node);

			for child in children.iter().rev() {
				self.stack.push((*child, false));
			}
		}

		None
	}
}

// pub struct SceneRBfsIterator {
// 	pub(crate) nodes: Vec<NodeId>,
// 	pub(crate) index: usize
// }

// impl Iterator for SceneRBfsIterator {
// 	type Item = NodeId;

// 	fn next(&mut self) -> Option<Self::Item> {
// 		if self.index < self.nodes.len() {
// 			let node = self.nodes[self.index];
// 			self.index += 1;
			
// 			return Some(node);
// 		}

// 		None
// 	}
// }
