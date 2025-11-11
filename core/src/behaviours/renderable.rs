use fatum_graphics::render::RenderObject;
use fatum_scene::{Node, NodeBehaviour};

pub trait Renderable: NodeBehaviour {
	fn render(delta: std::time::Duration);
}
