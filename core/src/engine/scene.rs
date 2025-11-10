use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, Mutex}};

use fatum_graphics::platform::GraphicsPlatform;
use fatum_resources::ResourcePlatform;
use fatum_scene::{Node, NodeBehaviour, SceneTree};
use signals2::Connect2;

use crate::{Application, CoreEngine, GraphicsEngine};

pub struct SceneEngine<P: GraphicsPlatform> {
	graphics: Rc<RefCell<GraphicsEngine<P>>>,
	scenes: HashMap<usize, Arc<Mutex<SceneTree>>>,
}

impl<P> SceneEngine<P> where P: GraphicsPlatform {
	pub fn new(graphics: Rc<RefCell<GraphicsEngine<P>>>) -> Self {
		log::info!("Created scene engine");

		Self {
			graphics,
			scenes: HashMap::new()
		}
	}

	pub fn scene(&self, output_index: usize) -> Option<Arc<Mutex<SceneTree>>> {
		self.scenes.get(&output_index).map_or(None, |v| Some(v.clone()))
	}

	pub fn set_scene(&mut self, output_index: usize, scene: Arc<Mutex<SceneTree>>) -> Option<bool> {
		let mut graphics = self.graphics.borrow_mut();

		let queue = graphics.get_output(output_index)?;

		fn node_ready(scene: Arc<Mutex<SceneTree>>, node: &mut Node) {
			let mut locked = scene.lock().unwrap();
			let children = locked.children(node.id());

			for child in children {
				let child_node = locked.node_mut(child)
						.expect("we are in deep shit");

				// TODO also do some shit like adding renderable nodes to queue i think
				
				node_ready(scene.clone(), child_node);
			}

			node.ready();
		}
		
		{
			// process root node separately as it cannot be mutable
			let mut locked = scene.lock().unwrap();
			let children = locked.children(0);

			for child in children {
				let child_node = locked.node_mut(child)
						.expect("we are in deep shit");
				
				node_ready(scene.clone(), child_node);
			}
		}

		if let Ok(scene) = scene.lock() {
			scene.node_added.connect(|scene, node| {
				// TODO add to queue
			});

			scene.node_removed.connect(|scene, node| {
				// TODO remove from queue
			});
		}

		self.scenes.insert(output_index, scene);
		Some(true)
	}
}
