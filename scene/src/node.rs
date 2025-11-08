use std::{rc::Rc, sync::{Arc, Mutex, atomic::Ordering}};

use rand::{Rng, distr::{Alphabetic, SampleString}};

use crate::SceneTree;

pub struct Node<'a> {
	id: u32,
	name: String,

	scene: Option<Arc<Mutex<SceneTree<'a>>>>,
	parent: Option<&'a mut Node<'a>>,
	children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
	pub const ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

	pub fn new(children: Vec<Node<'a>>) -> Self {
		Self {
			id: Self::ID_COUNTER.fetch_add(1, Ordering::Relaxed),
			name: Alphabetic.sample_string(&mut rand::rng(), 12),
			scene: None,
			parent: None,
			children
		}
	}

	pub fn id(&self) -> u32 { self.id }
	
	pub fn name(&self) -> &str { &self.name }
	pub fn set_name(&mut self, name: &str) { self.name = name.to_string() }

	pub fn scene(&self) -> Option<Arc<Mutex<SceneTree<'a>>>> { self.scene.clone() }
	pub fn parent(&'a self) -> Option<&Node> { self.parent.as_deref() }
	pub fn children(&self) -> &'a [Node] { &self.children }

	pub fn child_at(&'a self, index: usize) -> Option<&Node> {
		Some(&self.children[index])
	}

	pub fn child_by_name(&'a self, name: &str) -> Option<&Node> {
		for child in &self.children {
			if child.name == name {
				return Some(child);
			}
		}

		None
	}

	pub fn add_child(&mut self, node: Node<'a>) {
		self.children.push(node);
	}
}

impl<'a> NodeBehaviour<'a> for Node<'a> {
	fn enter_scene(&mut self, scene: Arc<Mutex<SceneTree<'a>>>, parent: Option<&'a mut Node<'a>>) {
		self.scene = Some(scene);
		self.parent = parent;
	}

	fn exit_scene(&mut self) {}
	fn ready(&mut self) {}
	fn update(&mut self, delta: std::time::Duration) {}
}

pub trait NodeBehaviour<'a> {
	fn enter_scene(&mut self, scene: Arc<Mutex<SceneTree<'a>>>, parent: Option<&'a mut Node<'a>>);
	fn exit_scene(&mut self);
	fn ready(&mut self);

	fn update(&mut self, delta: std::time::Duration);
}
