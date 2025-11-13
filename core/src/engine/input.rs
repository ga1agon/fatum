use std::{cell::RefCell, collections::HashMap, rc::Rc};

use fatum_graphics::{Window, platform::{GraphicsPlatform, opengl::OpenGlWindow}};
use glfw::Action;
use num_enum::FromPrimitive;

use crate::{GraphicsEngine, input::{self, ActionMap, Input, InputMap}};

pub struct InputEngine<P: GraphicsPlatform> {
	graphics: Rc<RefCell<GraphicsEngine<P>>>,
	inputs: HashMap<usize, Rc<RefCell<Input>>>
}

impl<P> InputEngine<P> where P: GraphicsPlatform {
	pub fn new(graphics: Rc<RefCell<GraphicsEngine<P>>>) -> Self {
		log::info!("Created input engine");
		
		Self {
			graphics,
			inputs: HashMap::new()
		}
	}

	pub fn create_input(&mut self, output_index: usize) -> Option<()> {
		log::info!("Creating input for output {}", output_index);

		let mut graphics = self.graphics.borrow_mut();
		let queue = graphics.get_output(output_index)?;

		let input = Rc::new(RefCell::new(Input::new()));
		let targets = queue.targets();

		self.inputs.insert(output_index, input.clone());

		for target in targets {
			if let Some(target) = queue.get_target_mut(target) {
				if let Some(window_target) = target.as_any_mut().downcast_mut::<OpenGlWindow>() {
					let input1 = input.clone();

					window_target.window_impl.set_key_callback(move |_, key, scancode, action, modifiers| {
						match action {
							Action::Press => input1.borrow_mut().key_down.emit((input::Key::from_primitive(key as i32), modifiers)),
							Action::Release => input1.borrow_mut().key_up.emit((input::Key::from_primitive(key as i32), modifiers)),
							Action::Repeat => input1.borrow_mut().key_repeat.emit((input::Key::from_primitive(key as i32), modifiers))
						};
					});

					let input2 = input.clone();

					window_target.window_impl.set_mouse_button_callback(move |_, button, action, modifiers| {
						match action {
							Action::Press => input2.borrow_mut().mouse_button_down.emit((input::MouseButton::from_primitive(button as i8), modifiers)),
							Action::Release => input2.borrow_mut().mouse_button_up.emit((input::MouseButton::from_primitive(button as i8), modifiers)),
							_ => {}
						}
					});
				}
			}
		}

		Some(())
	}

	pub fn input(&self, output_index: usize) -> Option<Rc<RefCell<Input>>> {
		if let Some(input) = self.inputs.get(&output_index) {
			return Some(input.clone())
		}

		None
	}

	pub fn create_input_map(&mut self, output_index: usize, action_map: ActionMap) -> Option<InputMap> {
		let mut input = self.input(output_index);

		if input.is_none() {
			if self.create_input(output_index).is_none() {
				return None
			}

			input = self.input(output_index);
		}

		Some(InputMap::new(input.clone().unwrap(), action_map))
	}
}
