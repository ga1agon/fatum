use fatum_scene::{NodeComponent, NodeId, SharedSceneGraph};

#[derive(NodeComponent)]
pub struct UiElement {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,
	draw_function: Box<dyn Fn(std::time::Duration, &Self, &mut dear_imgui_rs::Ui) -> ()>
}

impl UiElement {
	pub fn new<F: Fn(std::time::Duration, &Self, &mut dear_imgui_rs::Ui) -> () + 'static>(draw_function: F) -> Self {
		Self {
			owner: Default::default(),
			scene: Default::default(),
			draw_function: Box::new(draw_function)
		}
	}

	pub fn draw(&self, delta: std::time::Duration, ui: &mut dear_imgui_rs::Ui) {
		(self.draw_function)(delta, self, ui);
	}
}
