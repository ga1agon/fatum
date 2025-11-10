use fatum_graphics::render::RenderObject;
use fatum_scene::{Node, NodeBehaviour};
use fatum_signals::SignalDispatcher;

pub trait Renderable: NodeBehaviour {
	fn render(delta: std::time::Duration);
}
