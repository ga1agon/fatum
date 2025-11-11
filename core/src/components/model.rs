use std::{rc::Rc, sync::{Arc, Mutex}};

use fatum_graphics::render::RenderObject;
use fatum_scene::{NodeComponent, NodeId, SceneGraph, SharedSceneGraph};

use crate::components;

#[derive(NodeComponent)]
pub struct Model {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,
	model: Rc<Box<fatum_graphics::Model>>
}

impl Model {
	pub fn new(model: Rc<Box<fatum_graphics::Model>>) -> Self {
		Self {
			owner: 0,
			scene: None,
			model,
		}
	}

	pub fn model(&self) -> Rc<Box<fatum_graphics::Model>> { self.model.clone() }
	pub fn set_model(&mut self, model: Rc<Box<fatum_graphics::Model>>) {
		self.model = model.clone();

		if self.owner == 0 {
			return;
		}

		let scene = self.scene.clone().unwrap();

		if let Ok(mut scene) = scene.write() {
			let owner = scene.node_mut(self.owner).unwrap();

			if !owner.remove_component::<components::Model>() {
				log::warn!("Somehow, this component isn't owned by its owner");
				return;
			}

			owner.add_component(Box::new(components::Model::new(model)));
		}
	}
}

impl Into<RenderObject> for Model {
	fn into(self) -> RenderObject {
		RenderObject::with_id(self.owner as u64, self.model.clone())
	}
}

impl Into<RenderObject> for &Model {
	fn into(self) -> RenderObject {
		RenderObject::with_id(self.owner as u64, self.model.clone())
	}
}
