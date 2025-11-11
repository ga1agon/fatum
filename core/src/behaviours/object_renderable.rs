use fatum_graphics::render::RenderObject;
use fatum_scene::NodeBehaviour;
use fatum_signals::SignalDispatcher;

use crate::behaviours::Renderable;

pub struct ObjectRenderable {
	dispatcher: SignalDispatcher,
	object: RenderObject
}

impl ObjectRenderable {
	pub fn new(object: RenderObject) -> Self {
		Self {
			dispatcher: SignalDispatcher::new(),
			object
		}
	}
}

impl NodeBehaviour for ObjectRenderable {
	fn setup(&mut self) {
		
	}
	
	fn dispatcher(&self) -> &SignalDispatcher {
		todo!()
	}

	fn as_any(&self) -> &dyn std::any::Any { self }
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
