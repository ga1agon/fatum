use fatum_scene::NodeBehaviour;
use crate::{nodes::Node2D, resources::ResTexture2D};

#[derive(NodeBehaviour)]
pub struct Sprite2D {
	base: Node2D,
	texture: ResTexture2D
}
