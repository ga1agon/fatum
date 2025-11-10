use std::{collections::HashMap, rc::Rc, sync::{Arc, Mutex}};

#[cfg(feature = "signals")]
use signals2::{Emit2, Signal};

use crate::{Node, NodeBehaviour};

pub struct SceneTree {
	this: Option<Arc<Mutex<Self>>>,

	nodes: HashMap<u32, Node>,
	//behaviours: HashMap<u32, *const dyn NodeBehaviour>, // i <3 pointers
	child_parent: HashMap<u32, u32>,
	parent_children: HashMap<u32, Vec<u32>>,

	root: u32,

	#[cfg(feature = "signals")] pub node_added: Signal<(*const Self, *const Node)>,
	#[cfg(feature = "signals")] pub node_removed: Signal<(*const Self, *const Node)>,
}

impl SceneTree {
	pub fn new() -> Arc<Mutex<Self>> {
		let root = Node::with_id_name(0, "Root");

		let this = Arc::new(Mutex::new(Self {
			this: None,
			nodes: HashMap::from([
				(root.id(), root)
			]),
			//behaviours: HashMap::new(),
			child_parent: HashMap::new(),
			parent_children: HashMap::new(),
			root: 0,
			#[cfg(feature = "signals")] node_added: Signal::new(),
			#[cfg(feature = "signals")] node_removed: Signal::new(),
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

	pub fn behaviour(&self, id: u32) -> Option<&dyn NodeBehaviour> {
		self.nodes.get(&id).map(|n| n as &dyn NodeBehaviour)
	}

	// pub fn behaviour(&self, id: u32) -> Option<&Box<dyn NodeBehaviour>> {
	// 	self.behaviours.get(&id)
	// }

	// pub fn set_behaviour(&mut self, id: u32, behaviour: Option<Box<dyn NodeBehaviour>>) {
	// 	if behaviour.is_none() {
	// 		self.behaviours.remove(&id);
	// 	} else {
	// 		self.behaviours.insert(id, behaviour.unwrap());
	// 	}
	// }

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

		#[cfg(feature = "signals")]
		self.node_added.emit(self, &node);

		//let a = &node as &dyn NodeBehaviour;
		//self.behaviours.insert(node.id(), a);
		self.nodes.insert(node.id(), node);
	}

	pub fn remove_node(&mut self, node: Node) {
		let self_ptr = self as *const Self;

		if let Some(children) = self.parent_children.get_mut(&node.id()) {
			for child in children {
				let child_node = self.nodes.get_mut(child).expect("A node has children that are not in the scene");
				child_node.exit_scene();

				#[cfg(feature = "signals")]
				self.node_removed.emit(self_ptr, child_node);

				self.nodes.remove(child);
			}
		}

		#[cfg(feature = "signals")]
		self.node_removed.emit(self, &node);

		self.nodes.remove(&node.id());
	}
}
