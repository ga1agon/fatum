use std::{collections::HashMap, rc::Rc, sync::{Arc, Mutex}, vec};

use fatum_signals::StaticSignal;

use crate::{Node, NodeBehaviour, NodeComponent, NodeId};

pub struct SceneGraph {
	this: Option<Arc<Mutex<Self>>>,

	nodes: HashMap<NodeId, Node>,
	// components: HashMap<NodeId, Vec<*const dyn NodeComponent>>,
	// behaviours: HashMap<NodeId, Vec<*const dyn NodeBehaviour>>, // i <3 pointers
	child_parent: HashMap<NodeId, NodeId>,
	parent_children: HashMap<NodeId, Vec<NodeId>>,

	root: NodeId,

	pub node_added: StaticSignal<(*const Self, *const Node)>,
	pub node_removed: StaticSignal<(*const Self, *const Node)>,
}

impl SceneGraph {
	pub fn new() -> Arc<Mutex<Self>> {
		let root = Node::with_id_name(0, "SceneRoot");

		let this = Arc::new(Mutex::new(Self {
			this: None,
			nodes: HashMap::from([
				(root.id(), root)
			]),
			// components: HashMap::new(),
			// behaviours: HashMap::new(),
			child_parent: HashMap::new(),
			parent_children: HashMap::new(),
			root: 0,
			node_added: StaticSignal::new(),
			node_removed: StaticSignal::new(),
		}));

		{
			// TODO this is extremely concerning
			let mut scene = this.lock().unwrap();
			scene.this = Some(this.clone());
			scene.nodes.get_mut(&0).unwrap().enter_scene(0, this.clone());
		}

		this
	}

	pub fn root(&self) -> &Node {
		self.nodes.get(&self.root).unwrap()
	}

	pub fn node(&self, id: NodeId) -> Option<&Node> {
		self.nodes.get(&id)
	}

	pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
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

	// pub fn behaviour(&self, id: u32) -> Option<&dyn NodeBehaviour> {
	// 	self.nodes.get(&id).map(|n| n as &dyn NodeBehaviour)
	// }

	// pub fn components(&self, id: NodeId) -> Option<&Vec<*const dyn NodeComponent>> {
	// 	self.components.get(&id)
	// }

	// pub fn behaviours(&self, id: NodeId) -> Option<&Vec<*const dyn NodeBehaviour>> {
	// 	self.behaviours.get(&id)
	// 		// .map_or_else(|| {
	// 		// 	Vec::new() as Vec<Box<dyn NodeBehaviour>>
	// 		// }, |v| {
	// 		// 	v
	// 		// })
	// }

	// pub fn add_behaviour<B: NodeBehaviour + 'static>(&mut self, id: NodeId) {
	// 	let behaviour = node.as_any().downcast_ref::<B>()
	// 		.expect(format!("Node {} does not implement behaviour {}", std::any::type_name::<N>(), std::any::type_name::<B>()).as_str());

	// 	if let Some(behaviours) = self.behaviours.get_mut(&id) {
	// 		behaviours.push(behaviour); // this can very easily go out of scope and kill itself no?
	// 	} else {
	// 		self.behaviours.insert(id, vec![behaviour]);
	// 	}
	// }

	pub fn parent(&self, child: NodeId) -> NodeId {
		*self.child_parent.get(&child)
			.expect("How did we end up with a lost little lamb with no parent?")
	}

	pub fn children(&self, parent: NodeId) -> Vec<NodeId> {
		self.parent_children.get(&parent)
			.map_or_else(|| {
				Vec::new() as Vec<NodeId>
			}, |v| {
				v.clone()
			})
	}

	pub fn children_slice(&self, parent: NodeId) -> &[NodeId] {
		self.parent_children.get(&parent)
			.map_or_else(|| {
				&[] as &[NodeId]
			}, |v| {
				v.as_slice()
			})
	}

	pub fn child(&self, parent: NodeId, index: usize) -> Option<NodeId> {
		if let Some(children) = self.parent_children.get(&parent) {
			return children.get(index).copied();
		}

		None
	}

	pub fn add_node(&mut self, mut node: Node, parent: Option<NodeId>) {
		// let parent_node = if let Some(parent) = parent {
		// 	self.nodes.get(&parent)
		// } else {
		// 	None
		// };

		//node.set_id((self.nodes.len() + 1) as u32);
		let new_id = self.nodes.len() as NodeId;
		let parent = parent.unwrap_or_default(); // default == 0 == root
		
		if let Some(children) = self.parent_children.get_mut(&parent) {
			children.push(new_id);
		} else {
			self.parent_children.insert(parent, vec![new_id]);
		}

		node.enter_scene(new_id, self.this.as_ref().unwrap().clone());

		self.node_added.emit((self, &node));

		self.nodes.insert(new_id, node);
	}

	pub fn remove_node(&mut self, node: Node) {
		let self_ptr = self as *const Self;

		if let Some(children) = self.parent_children.get_mut(&node.id()) {
			for child in children {
				let child_node = self.nodes.get_mut(child).expect("A node has children that are not in its scene");
				child_node.exit_scene();

				self.node_removed.emit((self_ptr, child_node));

				self.nodes.remove(child);
				// TODO this should be recursive
			}
		}

		self.node_removed.emit((self, &node));

		self.nodes.remove(&node.id());
		//self.behaviours.remove(&node.id());
	}
}
