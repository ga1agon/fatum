use std::{collections::{HashMap, VecDeque}, fmt::Debug, rc::Rc, sync::{Arc, Mutex, RwLock}, vec};

use fatum_signals::StaticSignal;

use crate::{Node, NodeComponent, NodeId, iterators::{SceneDfsIterator, ScenePostDfsIterator}};

pub type SharedSceneGraph = Arc<RwLock<SceneGraph>>;

pub struct SceneGraph {
	this: Option<SharedSceneGraph>,

	nodes: HashMap<NodeId, Node>,
	child_parent: HashMap<NodeId, NodeId>,
	parent_children: HashMap<NodeId, Vec<NodeId>>,

	root: NodeId,

	pub node_added: StaticSignal<(*const Self, *const Node)>,
	pub node_removed: StaticSignal<(*const Self, *const Node)>,
	pub node_component_added: StaticSignal<(*const Node, *const Box<dyn NodeComponent>)>,
	pub node_component_removed: StaticSignal<(*const Node, *const Box<dyn NodeComponent>)>,
}

impl SceneGraph {
	pub fn new() -> SharedSceneGraph {
		let root = Node::with_id_name(0, "SceneRoot");

		let this = Arc::new(RwLock::new(Self {
			this: None,
			nodes: HashMap::from([
				(root.id(), root)
			]),
			child_parent: HashMap::new(),
			parent_children: HashMap::new(),
			root: 0,
			node_added: StaticSignal::new(),
			node_removed: StaticSignal::new(),
			node_component_added: StaticSignal::new(),
			node_component_removed: StaticSignal::new()
		}));

		{
			// TODO this is extremely concerning
			let mut scene = this.write().unwrap();
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

	pub fn add_node(&mut self, mut node: Node, parent: Option<NodeId>) -> NodeId {
		let new_id = self.nodes.len() as NodeId;
		let parent = parent.unwrap_or_default(); // default == 0 == root
		
		if let Some(children) = self.parent_children.get_mut(&parent) {
			children.push(new_id);
		} else {
			self.parent_children.insert(parent, vec![new_id]);
		}

		self.child_parent.insert(new_id, parent);

		node.enter_scene(new_id, self.this.as_ref().unwrap().clone());

		node.component_added.connect(|args| {
			unsafe {
				let node = &*args.0;

				let scene = node.scene().unwrap();
				if let Ok(scene) = scene.read() {
					scene.node_component_added.emit((args.0, args.1));
				}
			}
		});

		node.component_removed.connect(|args| {
			unsafe {
				let node = &*args.0;

				let scene = node.scene().unwrap();
				if let Ok(scene) = scene.read() {
					scene.node_component_removed.emit((args.0, args.1));
				}
			}
		});

		self.node_added.emit((self, &node));
		self.nodes.insert(new_id, node);
		new_id
	}

	pub fn remove_node(&mut self, node: Node) {
		let self_ptr = self as *const Self;

		// get this node and all of its children with the post order DFS algorithm and remove them
		for node in ScenePostDfsIterator::new(self.this.clone().unwrap(), node.id()) {
			{
				if let Some(node) = self.nodes.get_mut(&node) {
					node.exit_scene();
					self.node_removed.emit((self_ptr, node));
				}
			}

			// TODO uhhhh not 100% sure if this won't hit some concurrency shit and mess up the iterator
			self.nodes.remove(&node);
		}
	}
}

impl Debug for SceneGraph {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("SceneGraph")
			.field("node_count", &self.nodes.len())
			.field("root", &self.root)
			.finish()
	}
}
