use crate::{nodes::Node2D, resources::ResTexture2D};

pub struct Sprite2D<'a> {
	base: Node2D<'a>,
	texture: ResTexture2D
}
