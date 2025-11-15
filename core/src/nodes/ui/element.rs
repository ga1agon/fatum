use fatum_scene::Node;

use crate::components;

pub struct UiElement {}

impl UiElement {
	pub fn new<F: Fn(std::time::Duration, &components::UiElement, &egui::Context) -> () + 'static>(draw_function: F) -> Node {
		let mut node = Node::new();

		let element = Box::new(components::UiElement::new(draw_function));

		node.add_component(element);
		node
	}
}
