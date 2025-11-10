use std::{any::Any, rc::Rc, sync::{Arc, Mutex, atomic::Ordering}};

use rand::{Rng, distr::{Alphabetic, SampleString}};

use crate::{NodeBehaviour, NodeComponent, SceneGraph, lock_opt_mutex_unchecked};

pub type NodeId = u32;

pub struct Node {
	id: NodeId,
	name: String,
	
	scene: Option<Arc<Mutex<SceneGraph>>>,
	components: Vec<Box<dyn NodeComponent>>,
	behaviours: Vec<Box<dyn NodeBehaviour>>,
}

impl Node {
	pub fn new() -> Self {
		Self::with_name(&Alphabetic.sample_string(&mut rand::rng(), 12))
	}

	pub fn with_name(name: &str) -> Self {
		Self::with_id_name(u32::MAX, name)
	}

	pub fn with_id_name(id: u32, name: &str) -> Self {
		Self {
			id,
			name: name.to_string(),
			scene: None,
			components: vec![],
			behaviours: vec![]
		}
	}

	pub fn id(&self) -> NodeId { self.id }

	pub fn name(&self) -> &str { &self.name }
	pub fn set_name(&mut self, name: &str) { self.name = name.to_string() }

	pub fn scene(&self) -> Option<Arc<Mutex<SceneGraph>>> { self.scene.clone() }
	pub fn parent(&self) -> NodeId { lock_opt_mutex_unchecked(&self.scene).parent(self.id) }
	pub fn children(&self) -> Vec<u32> { lock_opt_mutex_unchecked(&self.scene).children(self.id) }
	//pub fn components(&self) -> Vec<Box<dyn NodeComponent>> { lock_opt_mutex_unchecked(&self.scene).components(self.id) }

	pub fn component<T: NodeComponent>(&self) -> Option<&T> {
		for component in &self.components {
			let component_any = component.as_any();

			if component_any.is::<T>() {
				return component_any.downcast_ref();
			}
		}

		None
	}

	pub fn component_mut<T: NodeComponent>(&mut self) -> Option<&mut T> {
		for component in &mut self.components {
			let component_any = component.as_any_mut();

			if component_any.is::<T>() {
				return component_any.downcast_mut();
			}
		}

		None
	}

	pub fn components(&self) -> &Vec<Box<dyn NodeComponent>> { &self.components }
	pub fn add_component(&mut self, component: Box<dyn NodeComponent>) { self.components.push(component) }

	pub fn behaviour<T: NodeComponent>(&self) -> Option<&T> {
		for behaviour in &self.behaviours {
			let behaviour_any = behaviour.as_any();

			if behaviour_any.is::<T>() {
				return behaviour_any.downcast_ref();
			}
		}

		None
	}

	pub fn behaviour_mut<T: NodeComponent>(&mut self) -> Option<&mut T> {
		for behaviour in &mut self.behaviours {
			let behaviour_any = behaviour.as_any_mut();

			if behaviour_any.is::<T>() {
				return behaviour_any.downcast_mut();
			}
		}

		None
	}

	pub fn behaviours(&self) -> &Vec<Box<dyn NodeBehaviour>> { &self.behaviours }
	pub fn add_behaviour(&mut self, mut behaviour: Box<dyn NodeBehaviour>) {
		behaviour.setup();
		self.behaviours.push(behaviour);
	}

	pub fn enter_scene(&mut self, id: NodeId, scene: Arc<Mutex<SceneGraph>>) {
		self.id = id;
		self.scene = Some(scene);
	}

	pub fn exit_scene(&mut self) {
		self.id = 0;
		self.scene = None;
	}

	pub fn as_any(&self) -> &dyn std::any::Any { self }
}

// behaviours are like components but not only store data!!!
//pub trait NodeBehaviour {}
//impl<T: 'static> NodeBehaviour for T {}

// impl Node {
// 	//pub const ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

// 	pub fn new() -> Self {
// 		Self::with_name(&Alphabetic.sample_string(&mut rand::rng(), 12))
// 	}

// 	pub fn with_name(name: &str) -> Self {
// 		Self::with_id_name(u32::MAX, name)
// 	}

// 	pub fn with_id_name(id: u32, name: &str) -> Self {
// 		Self {
// 			id,
// 			name: name.to_string(),
// 			components: vec![],
// 			scene: None,
// 			//parent: None,
// 			//children
// 		}
// 	}

// 	pub fn id(&self) -> u32 { self.id }
// 	pub(crate) fn set_id(&mut self, id: u32) { self.id = id }

// 	pub fn name(&self) -> &str { &self.name }
// 	pub fn set_name(&mut self, name: &str) { self.name = name.to_string() }

// 	pub fn scene(&self) -> Option<Arc<Mutex<SceneGraph>>> { self.scene.clone() }
// 	pub fn parent(&self) -> u32 { lock_opt_mutex_unchecked(&self.scene).parent(self.id) }
// 	pub fn children(&self) -> Vec<u32> { lock_opt_mutex_unchecked(&self.scene).children(self.id) }

// 	// pub fn child_at(&self, index: usize) -> Option<&u32> {
// 	// 	self.children.get(index)
// 	// }

// 	// pub fn child_by_name(&self, name: &str) -> Option<&Node> {
// 	// 	for child in &self.children {
// 	// 		if child.name == name {
// 	// 			return Some(child);
// 	// 		}
// 	// 	}

// 	// 	None
// 	// }

// 	// pub fn add_child(&mut self, node: Node<'a>) {
// 	// 	self.children.push(node);
// 	// }
// }

// impl NodeBehaviour for Node {
// 	fn enter_scene(&mut self, scene: Arc<Mutex<SceneGraph>>) {
// 		self.scene = Some(scene);
// 	}

// 	fn exit_scene(&mut self) {
// 		self.scene = None;
// 	}

// 	fn ready(&mut self) {}
// 	fn update(&mut self, delta: std::time::Duration) {}
// }

// pub trait NodeBehaviour {
// 	//fn base(&self) -> &Node;

// 	fn enter_scene(&mut self, scene: Arc<Mutex<SceneGraph>>);
// 	fn exit_scene(&mut self);
// 	fn ready(&mut self);

// 	fn update(&mut self, delta: std::time::Duration);
// }
