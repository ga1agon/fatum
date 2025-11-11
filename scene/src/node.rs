use std::{any::Any, collections::HashMap, rc::Rc, sync::{Arc, Mutex, atomic::Ordering}};

use fatum_signals::{Signal, SignalDispatcher, StaticSignal};
use rand::{Rng, distr::{Alphabetic, SampleString}};

use crate::{NodeBehaviour, NodeComponent, SceneGraph, SharedSceneGraph, lock_opt_mutex_unchecked};

pub type NodeId = u32;

pub struct Node {
	id: NodeId,
	name: String,
	
	scene: Option<SharedSceneGraph>,
	components: Vec<Box<dyn NodeComponent>>,

	pub component_added: StaticSignal<(*const Node, *const Box<dyn NodeComponent>)>,
	pub component_removed: StaticSignal<(*const Node, *const Box<dyn NodeComponent>)>,

	signals: HashMap<String, Box<dyn Signal>>
}

impl Node {
	pub fn new() -> Self {
		Self::with_name(&Alphabetic.sample_string(&mut rand::rng(), 12))
	}

	pub fn with_name(name: &str) -> Self {
		Self::with_id_name(u32::MAX, name)
	}

	pub fn with_id_name(id: u32, name: &str) -> Self {
		let mut this = Self {
			id,
			name: name.to_string(),
			scene: None,
			components: vec![],
			component_added: StaticSignal::new(),
			component_removed: StaticSignal::new(),
			signals: HashMap::new()
		};

		this.create_signal::<()>("enter_tree");
		this.create_signal::<()>("exit_tree");
		this.create_signal::<()>("ready");

		this.create_signal::<std::time::Duration>("update");
		this.create_signal_mut::<std::time::Duration>("$update");

		this
	}

	pub fn id(&self) -> NodeId { self.id }

	pub fn name(&self) -> &str { &self.name }
	pub fn set_name(&mut self, name: &str) { self.name = name.to_string() }

	pub fn scene(&self) -> Option<SharedSceneGraph> { self.scene.clone() }
	pub fn parent(&self) -> NodeId { self.scene.as_ref().unwrap().read().unwrap().parent(self.id) }
	pub fn children(&self) -> Vec<u32> { self.scene.as_ref().unwrap().read().unwrap().children(self.id) }
	//pub fn components(&self) -> Vec<Box<dyn NodeComponent>> { lock_opt_mutex_unchecked(&self.scene).components(self.id) }

	pub fn component<T: NodeComponent>(&self) -> Option<&T> {
		// TODO store in a HashMap for fast lookup
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
	// TODO check if component already exists
	pub fn add_component(&mut self, mut component: Box<dyn NodeComponent>) {
		if let Some(scene) = &self.scene {
			component.enter_scene(self.id, scene.clone());
		}

		self.component_added.emit((self, &component));
		self.components.push(component);
	}

	pub fn remove_component<T: NodeComponent>(&mut self) -> bool {
		for i in 0..self.components.len() {
			let component = &self.components[i];

			if component.as_any().is::<T>() {
				self.component_removed.emit((self, component));
				self.components.remove(i);
				return true;
			}
		}

		false
	}

	pub fn enter_scene(&mut self, id: NodeId, scene: SharedSceneGraph) {
		self.id = id;
		self.scene = Some(scene.clone());

		for component in &mut self.components {
			component.enter_scene(id, scene.clone());
		}

		self.emit("enter_scene", ());
	}

	pub fn exit_scene(&mut self) {
		self.emit("exit_scene", ());

		self.id = 0;
		self.scene = None;
		
		self.component_added.clear();
		self.component_removed.clear();

		for component in &mut self.components {
			component.exit_scene();
		}
	}

	pub fn ready(&self) {
		self.emit("ready", ());
	}

	pub fn as_any(&self) -> &dyn std::any::Any { self }

	// signals (kinda messy :/)
	pub fn create_signal<Args: 'static>(&mut self, name: &str) {
		let signal = StaticSignal::<(*const Self, Args)>::new();
		self.signals.insert(name.to_string(), Box::new(signal));
	}

	pub fn create_signal_mut<Args: 'static>(&mut self, name: &str) {
		let signal_mut = StaticSignal::<(*mut Self, Args)>::new();
		self.signals.insert(name.to_string(), Box::new(signal_mut));
	}

	pub fn connect<Args: 'static, F: Fn(&(*const Self, Args)) -> () + 'static>(&mut self, name: &str, handler: F) {
		let signal = self.signals.get_mut(&name.to_string())
			.expect(format!("No such signal: {}", name).as_str());

		let handler = Box::new(Box::new(handler) as Box<dyn Fn(&(*const Self, Args))>) as Box<dyn Any>;
		signal.connect_any(handler);
	}

	pub fn connect_mut<Args: 'static, F: Fn(&(*mut Self, Args)) -> () + 'static>(&mut self, name: &str, handler: F) {
		let signal = self.signals.get_mut(&name.to_string())
			.expect(format!("No such signal: {}", name).as_str());

		let handler = Box::new(Box::new(handler) as Box<dyn Fn(&(*mut Self, Args))>) as Box<dyn Any>;
		signal.connect_any(handler);
	}

	pub fn disconnect<Args: 'static, F: Fn(&(*const Self, Args)) -> () + 'static>(&mut self, name: &str, handler: F) {
		let signal = self.signals.get_mut(&name.to_string())
			.expect(format!("No such signal: {}", name).as_str());

		let handler = Box::new(Box::new(handler) as Box<dyn Fn(&(*const Self, Args))>) as Box<dyn Any>;
		signal.disconnect_any(handler);
	}

	pub fn disconnect_mut<Args: 'static, F: Fn(&(*mut Self, Args)) -> () + 'static>(&mut self, name: &str, handler: F) {
		let signal = self.signals.get_mut(&name.to_string())
			.expect(format!("No such signal: {}", name).as_str());

		let handler = Box::new(Box::new(handler) as Box<dyn Fn(&(*mut Self, Args))>) as Box<dyn Any>;
		signal.disconnect_any(handler);
	}

	pub fn emit<Args: 'static>(&self, name: &str, args: Args) {
		let args = (self as *const Self, args);

		if let Some(signal) = self.signals.get(&name.to_string()) {
			signal.emit_any(&args);
		}
	}

	pub fn emit_mut<Args: 'static>(&mut self, name: &str, args: Args) {
		let args = (self as *mut Self, args);

		if let Some(signal) = self.signals.get(&name.to_string()) {
			signal.emit_any(&args);
		}
	}

	// pub fn emit_strict<Args: 'static>(&self, name: &str, args: Args) {
	// 	let args = (self as *const Self, args);

	// 	let signal = self.signals.get(&name.to_string())
	// 		.expect(format!("No such signal: {}", name).as_str());
		
	// 	signal.emit_any(&args);
	// }
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
