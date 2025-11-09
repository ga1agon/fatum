use fatum_scene::NodeBehaviour;

use crate::{nodes::Node2D, resources::ResTexture2D};

pub struct Sprite2D<'a> {
	base: Node2D<'a>,
	texture: ResTexture2D
}

impl NodeBehaviour for Sprite2D {
	fn enter_scene(&mut self, scene: std::sync::Arc<std::sync::Mutex<fatum_scene::SceneTree<'a>>>, parent: Option<&'a mut fatum_scene::Node<'a>>) {
		todo!()
	}

	fn exit_scene(&mut self) {
		todo!()
	}

	fn ready(&mut self) {
		todo!()
	}

	fn update(&mut self, delta: std::time::Duration) {
		todo!()
	}
}
