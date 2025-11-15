use fatum_scene::Node;

use crate::components;

pub struct UiWindow {}

impl UiWindow {
	pub fn new<F: Fn(std::time::Duration, &components::UiElement, &mut egui::Ui) -> () + 'static>(title: String, draw_function: F) -> Node {
		let mut node = Node::new();

		let element = Box::new(components::UiElement::new(move |delta, element, ctx| {
			egui::Window::new(title.as_str()).auto_sized().show(ctx, |ui| {
				draw_function(delta, element, ui);
			});
		}));

		node.add_component(element);
		node
	}
}
