use std::{cell::RefCell, collections::HashMap, rc::Rc};

use fatum_graphics::{Window, platform::{GraphicsPlatform, opengl::OpenGlWindow}};
use glam::Vec2;
use glfw::{Action, Context};
use num_enum::FromPrimitive;

use crate::{GraphicsEngine, input::{self, ActionMap, Input, InputMap, MouseScrollWheel}};

pub struct InputEngine<P: GraphicsPlatform> {
	graphics: Rc<RefCell<GraphicsEngine<P>>>,
	inputs: HashMap<usize, Rc<RefCell<Input>>>,
	input_maps: Vec<Rc<RefCell<InputMap>>>
}

impl<P> InputEngine<P> where P: GraphicsPlatform {
	pub fn new(graphics: Rc<RefCell<GraphicsEngine<P>>>) -> Self {
		log::info!("Created input engine");
		
		Self {
			graphics,
			inputs: HashMap::new(),
			input_maps: Vec::new()
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

					let input3 = input.clone();
					window_target.window_impl.set_scroll_callback(move |_, delta_x, delta_y| {
						let scroll_wheel = if delta_y > 0.0 {
							MouseScrollWheel::Up
						} else if delta_y < 0.0 {
							MouseScrollWheel::Down
						} else if delta_x > 0.0 {
							MouseScrollWheel::Right
						} else if delta_x < 0.0 {
							MouseScrollWheel::Left
						} else {
							MouseScrollWheel::None
						};

						input3.borrow_mut().mouse_scroll.emit(scroll_wheel);
					});

					let input4 = input.clone();
					let window_ptr = window_target.window_impl.window_ptr();

					window_target.window_impl.set_cursor_pos_callback(move |_, x, y| {
						let mut window_width = 0;
						let mut window_height = 0;

						unsafe {
							glfw::ffi::glfwGetWindowSize(window_ptr, &mut window_width as *mut i32, &mut window_height as *mut i32);
						}

						// we use UP = Y+, not Y-, so need to flip vertically
						let position = Vec2::new(x as f32, window_height as f32 - y as f32);
						input4.borrow_mut().cursor_position = position;
					});

					input.borrow_mut().cursor_mode_set.connect(move |mode| {
						unsafe {
							glfw::ffi::glfwSetInputMode(window_ptr, glfw::ffi::GLFW_CURSOR, *mode as i32);
						}
					});
				}
			}
		}

		Some(())
	}

	pub fn input(&self, output_index: usize) -> Option<Rc<RefCell<Input>>> {
		self.inputs.get(&output_index).cloned()
	}

	pub fn create_input_map(&mut self, output_index: usize, action_map: ActionMap) -> Option<Rc<RefCell<InputMap>>> {
		let mut input = self.input(output_index);

		if input.is_none() {
			if self.create_input(output_index).is_none() {
				return None
			}

			input = self.input(output_index);
		}

		let input_map = Rc::new(RefCell::new(InputMap::new(input.clone().unwrap(), action_map)));
		self.input_maps.push(input_map.clone());

		Some(input_map.clone())
	}

	pub fn process(&mut self) {
		for input_map in &self.input_maps {
			input_map.borrow_mut().process();
		}
	}
}
