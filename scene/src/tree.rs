use std::sync::RwLockWriteGuard;

use serde::{Deserialize, Serialize};

use crate::{Node, NodeComponent, NodeId, SceneGraph, SharedSceneGraph};

pub struct NodeTreeEntry {
	pub components: Vec<Box<dyn NodeComponent>>,
	pub children: Vec<NodeTreeEntry>,
}

impl NodeTreeEntry {
	pub fn new() -> Self {
		Self {
			components: Vec::new(),
			children: Vec::new()
		}
	}
}

pub struct NodeTree {
	pub root: NodeTreeEntry
}

impl NodeTree {
	pub fn new() -> Self {
		Self {
			root: NodeTreeEntry::new()
		}
	}

	pub fn instantiate(&self, scene: SharedSceneGraph, parent: Option<NodeId>) -> u32 {
		let mut scene = scene.write().unwrap();

		fn create_node(entry: &NodeTreeEntry) -> Node {
			let mut node = Node::new();

			for component in &entry.components {
				node.add_component((*component).clone_component());
			}
			
			node
		}

		fn add_node(scene: &mut RwLockWriteGuard<'_, SceneGraph>, entry: &NodeTreeEntry, parent: Option<NodeId>) -> u32 {
			let node = create_node(entry);
			let node = scene.add_node(node, parent);

			for child in &entry.children {
				add_node(scene, child, Some(node));
			}

			node
		}

		add_node(&mut scene, &self.root, parent)
	}
}
