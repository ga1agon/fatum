use std::{collections::HashMap, rc::Rc, sync::{Arc, Mutex}};

use crate::{Node, NodeBehaviour};

pub struct SceneTree {
	this: Option<Arc<Mutex<Self>>>,

	nodes: HashMap<u32, Node>,
	child_parent: HashMap<u32, u32>,
	parent_children: HashMap<u32, Vec<u32>>,

	root: u32
}

impl SceneTree {
	pub fn new() -> Arc<Mutex<Self>> {
		let root = Node::with_id_name(0, "Root", vec![]);

		let this = Arc::new(Mutex::new(Self {
			this: None,
			nodes: HashMap::from([
				(root.id(), root)
			]),
			child_parent: HashMap::new(),
			parent_children: HashMap::new(),
			root: 0
		}));

		{
			// TODO this is extremely concerning
			let mut scene = this.lock().unwrap();
			scene.this = Some(this.clone());
			scene.nodes.get_mut(&0).unwrap().enter_scene(this.clone());
		}

		this
	}

	pub fn root(&self) -> &Node {
		self.nodes.get(&self.root).unwrap()
	}

	pub fn node(&self, id: u32) -> Option<&Node> {
		self.nodes.get(&id)
	}

	pub fn node_mut(&mut self, id: u32) -> Option<&mut Node> {
		self.nodes.get_mut(&id)
	}

	pub fn node_by_name(&self, name: &str) -> Option<&Node> {
		for (_, node) in &self.nodes {
			if node.name() == name {
				return Some(node);
			}
		}

		None
	}

	pub fn node_by_name_mut(&mut self, name: &str) -> Option<&mut Node> {
		for (_, node) in &mut self.nodes {
			if node.name() == name {
				return Some(node);
			}
		}

		None
	}

	pub fn parent(&self, child: u32) -> u32 {
		*self.child_parent.get(&child)
			.expect("How did we end up with a lost little lamb with no parent?")
	}

	pub fn children(&self, parent: u32) -> Vec<u32> {
		self.parent_children.get(&parent)
			.map_or_else(|| {
				Vec::new() as Vec<u32>
			}, |v| {
				v.clone()
			})
	}

	pub fn children_slice(&self, parent: u32) -> &[u32] {
		self.parent_children.get(&parent)
			.map_or_else(|| {
				&[] as &[u32]
			}, |v| {
				v.as_slice()
			})
	}

	pub fn child(&self, parent: u32, index: usize) -> Option<u32> {
		if let Some(children) = self.parent_children.get(&parent) {
			return children.get(index).copied();
		}

		None
	}

	pub fn add_node(&mut self, mut node: Node, parent: Option<u32>) {
		// let parent_node = if let Some(parent) = parent {
		// 	self.nodes.get(&parent)
		// } else {
		// 	None
		// };

		node.set_id((self.nodes.len() + 1) as u32);
		
		let parent = parent.unwrap_or_default();
		
		if let Some(children) = self.parent_children.get_mut(&parent) {
			children.push(node.id());
		} else {
			self.parent_children.insert(parent, vec![node.id()]);
		}

		node.enter_scene(self.this.as_ref().unwrap().clone());
		self.nodes.insert(node.id(), node);
	}
}
