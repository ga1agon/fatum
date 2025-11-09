use std::{rc::Rc, sync::{Arc, Mutex, atomic::Ordering}};

use rand::{Rng, distr::{Alphabetic, SampleString}};

use crate::{SceneTree, lock_opt_mutex_unchecked};

pub struct Node {
	id: u32,
	name: String,

	scene: Option<Arc<Mutex<SceneTree>>>,
	//parent: Option<u32>,
	//children: Vec<u32>,
}

impl Node {
	//pub const ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

	pub fn new(children: Vec<u32>) -> Self {
		Self::with_name(&Alphabetic.sample_string(&mut rand::rng(), 12), children)
	}

	pub fn with_name(name: &str, children: Vec<u32>) -> Self {
		Self::with_id_name(u32::MAX, name, children)
	}

	pub fn with_id_name(id: u32, name: &str, children: Vec<u32>) -> Self {
		Self {
			id,
			name: name.to_string(),
			scene: None,
			//parent: None,
			//children
		}
	}

	pub fn id(&self) -> u32 { self.id }
	pub(crate) fn set_id(&mut self, id: u32) { self.id = id }
	
	pub fn name(&self) -> &str { &self.name }
	pub fn set_name(&mut self, name: &str) { self.name = name.to_string() }

	pub fn scene(&self) -> Option<Arc<Mutex<SceneTree>>> { self.scene.clone() }
	pub fn parent(&self) -> u32 { lock_opt_mutex_unchecked(&self.scene).parent(self.id) }
	pub fn children(&self) -> Vec<u32> { lock_opt_mutex_unchecked(&self.scene).children(self.id) }

	// pub fn child_at(&self, index: usize) -> Option<&u32> {
	// 	self.children.get(index)
	// }

	// pub fn child_by_name(&self, name: &str) -> Option<&Node> {
	// 	for child in &self.children {
	// 		if child.name == name {
	// 			return Some(child);
	// 		}
	// 	}

	// 	None
	// }

	// pub fn add_child(&mut self, node: Node<'a>) {
	// 	self.children.push(node);
	// }
}

impl NodeBehaviour for Node {
	fn enter_scene(&mut self, scene: Arc<Mutex<SceneTree>>) {
		self.scene = Some(scene);
	}

	fn exit_scene(&mut self) {
		self.scene = None;
	}

	fn ready(&mut self) {}
	fn update(&mut self, delta: std::time::Duration) {}
}

pub trait NodeBehaviour {
	fn enter_scene(&mut self, scene: Arc<Mutex<SceneTree>>);
	fn exit_scene(&mut self);
	fn ready(&mut self);

	fn update(&mut self, delta: std::time::Duration);
}
