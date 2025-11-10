use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, Mutex}};

use fatum_graphics::{platform::GraphicsPlatform, render::{RenderObject, RenderQueue}};
use fatum_resources::ResourcePlatform;
use fatum_scene::{Node, NodeBehaviour, SceneGraph};
use signals2::Connect2;

use crate::{Application, CoreEngine, GraphicsEngine, components::{Transform, Transform2D}};

pub struct SceneEngine<P: GraphicsPlatform> {
	graphics: Rc<RefCell<GraphicsEngine<P>>>,
	scenes: HashMap<usize, Arc<Mutex<SceneGraph>>>,
}

impl<P> SceneEngine<P> where P: GraphicsPlatform {
	pub fn new(graphics: Rc<RefCell<GraphicsEngine<P>>>) -> Self {
		log::info!("Created scene engine");

		Self {
			graphics,
			scenes: HashMap::new()
		}
	}

	pub fn scene(&self, output_index: usize) -> Option<Arc<Mutex<SceneGraph>>> {
		self.scenes.get(&output_index).map_or(None, |v| Some(v.clone()))
	}

	pub fn set_scene(&mut self, output_index: usize, scene: Arc<Mutex<SceneGraph>>) -> Option<bool> {
		let mut graphics = self.graphics.borrow_mut();

		let queue = graphics.get_output(output_index)?;

		fn node_ready(queue: &mut Box<dyn RenderQueue>, scene: Arc<Mutex<SceneGraph>>, node: &mut Node) {
			let mut locked = scene.lock().unwrap();
			let children = locked.children(node.id());

			for child in children {
				let child_node = locked.node_mut(child)
						.expect("we are in deep shit");

				// TODO also do some shit like adding renderable nodes to queue i think
				
				node_ready(queue, scene.clone(), child_node);
			}

			if let Some(render_object) = node.component::<RenderObject>()
				&& let Some(transform2d) = node.component::<Transform2D>()
			{
				queue.add_object(Rc::new(render_object), transform2d.local_matrix());
			}
			//node.ready();
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

		if let Ok(scene) = scene.lock().as_mut() {
			scene.node_added.connect(|(scene, node)| {
				// TODO add to queue
			});

			scene.node_removed.connect(|(scene, node)| {
				// TODO remove from queue
			});
		}

		self.scenes.insert(output_index, scene);
		Some(true)
	}
}
