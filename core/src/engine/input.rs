use std::{cell::RefCell, collections::HashMap, rc::Rc};

use fatum_graphics::{RenderWindow, platform::{GraphicsPlatform, opengl::OpenGlWindow}};
use fatum_resources::ResourceRef;
use glam::Vec2;
use num_enum::FromPrimitive;
use winit::{dpi::PhysicalPosition, event::{DeviceId, ElementState, KeyEvent, MouseScrollDelta}, keyboard::PhysicalKey, window::WindowId};

use crate::{GraphicsEngine, input::{self, ActionMap, Input, InputMap, MouseScroll}, resources::ResActionMap};

pub struct InputEngine<P: GraphicsPlatform> {
	graphics: Rc<RefCell<GraphicsEngine<P>>>,
	inputs: HashMap<usize, (Vec<WindowId>, Rc<RefCell<Input>>)>,
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
		let queue = graphics.output(output_index)?;

		let input = Rc::new(RefCell::new(Input::new()));
		let targets = queue.targets();

		let mut window_ids = Vec::new();

		for target in targets {
			if let Some(target) = queue.get_target_mut(target) {
				if let Some(window_target) = target.as_any_mut().downcast_mut::<OpenGlWindow>() {
					window_ids.push(window_target.wimpl().id());
					log::info!("Window {:?} in output {} will have input", window_target.wimpl().id(), output_index);
				}
			}
		}

		self.inputs.insert(output_index, (window_ids, input.clone()));

		// for target in targets {
		// 	if let Some(target) = queue.get_target_mut(target) {
		// 		if let Some(window_target) = target.as_any_mut().downcast_mut::<OpenGlWindow>() {
		// 			let input1 = input.clone();
		// 			window_target.window_impl.set_key_callback(move |_, key, scancode, action, modifiers| {
		// 				match action {
		// 					Action::Press => input1.borrow_mut().key_down.emit((input::Key::from_primitive(key as i32), modifiers)),
		// 					Action::Release => input1.borrow_mut().key_up.emit((input::Key::from_primitive(key as i32), modifiers)),
		// 					Action::Repeat => input1.borrow_mut().key_repeat.emit((input::Key::from_primitive(key as i32), modifiers))
		// 				};
		// 			});

		// 			let input2 = input.clone();
		// 			window_target.window_impl.set_mouse_button_callback(move |_, button, action, modifiers| {
		// 				match action {
		// 					Action::Press => input2.borrow_mut().mouse_button_down.emit((input::MouseButton::from_primitive(button as i8), modifiers)),
		// 					Action::Release => input2.borrow_mut().mouse_button_up.emit((input::MouseButton::from_primitive(button as i8), modifiers)),
		// 					_ => {}
		// 				}
		// 			});

		// 			let input3 = input.clone();
		// 			window_target.window_impl.set_scroll_callback(move |_, delta_x, delta_y| {
		// 				let scroll_wheel = if delta_y > 0.0 {
		// 					MouseScrollWheel::Up
		// 				} else if delta_y < 0.0 {
		// 					MouseScrollWheel::Down
		// 				} else if delta_x > 0.0 {
		// 					MouseScrollWheel::Right
		// 				} else if delta_x < 0.0 {
		// 					MouseScrollWheel::Left
		// 				} else {
		// 					MouseScrollWheel::None
		// 				};

		// 				input3.borrow_mut().mouse_scroll.emit(scroll_wheel);
		// 			});

		// 			let input4 = input.clone();
		// 			let window_ptr = window_target.window_impl.window_ptr();

		// 			window_target.window_impl.set_cursor_pos_callback(move |_, x, y| {
		// 				let mut window_width = 0;
		// 				let mut window_height = 0;

		// 				unsafe {
		// 					glfw::ffi::glfwGetWindowSize(window_ptr, &mut window_width as *mut i32, &mut window_height as *mut i32);
		// 				}

		// 				// we use UP = Y+, not Y-, so need to flip vertically
		// 				let position = Vec2::new(x as f32, window_height as f32 - y as f32);
		// 				input4.borrow_mut().cursor_position = position;
		// 			});

		// 			input.borrow_mut().cursor_mode_set.connect(move |mode| {
		// 				unsafe {
		// 					glfw::ffi::glfwSetInputMode(window_ptr, glfw::ffi::GLFW_CURSOR, *mode as i32);
		// 				}
		// 			});
		// 		}
		// 	}
		// }

		Some(())
	}

	pub fn input(&self, output_index: usize) -> Option<Rc<RefCell<Input>>> {
		if self.inputs.contains_key(&output_index) {
			let input = &self.inputs[&output_index];
			return Some(input.1.clone());
		}

		None
	}

	pub fn create_input_map(&mut self, output_index: usize, action_map: ResourceRef<ResActionMap>) -> Option<Rc<RefCell<InputMap>>> {
		let mut input = self.input(output_index);

		if input.is_none() {
			if self.create_input(output_index).is_none() {
				return None;
			}

			input = self.input(output_index);
		}

		assert!(input.is_some());

		let input = input.clone().unwrap();

		let input_map = Rc::new(RefCell::new(InputMap::new(input, action_map)));
		self.input_maps.push(input_map.clone());

		Some(input_map.clone())
	}

	pub fn process(&mut self) {
		for input_map in &self.input_maps {
			input_map.borrow_mut().process();
		}
	}

	pub fn on_keyboard_input(&mut self, window: WindowId, device: DeviceId, event: KeyEvent) {
		for (_, (window_ids, input)) in &self.inputs {
			if !window_ids.contains(&window) {
				continue;
			}

			let input = input.borrow_mut();

			match event.physical_key {
				PhysicalKey::Code(key_code) => {
					match event.state {
						ElementState::Pressed => {
							input.key_down.emit(key_code);
						},
						ElementState::Released => {
							input.key_up.emit(key_code);
						}
					}
				},
				_ => log::warn!("Unknown key pressed: {:?}", event.physical_key)
			}
		}
	}

	pub fn on_mouse_move(&mut self, window: WindowId, device: DeviceId, position: PhysicalPosition<f64>) {
		for (_, (window_ids, input)) in &self.inputs {
			if !window_ids.contains(&window) {
				continue;
			}

			
			input.borrow_mut().cursor_position = Vec2::new(position.x as f32, position.y as f32);
		}
	}

	pub fn on_mouse_input(&mut self, window: WindowId, device: DeviceId, button: winit::event::MouseButton, state: ElementState) {
		for (_, (window_ids, input)) in &self.inputs {
			if !window_ids.contains(&window) {
				continue;
			}

			let input = input.borrow_mut();

			match state {
				ElementState::Pressed => {
					input.mouse_button_down.emit(button);
				},
				ElementState::Released => {
					input.mouse_button_up.emit(button);
				}
			}
		}
	}

	pub fn on_mouse_scroll(&mut self, window: WindowId, device: DeviceId, delta: MouseScrollDelta) {
		for (_, (window_ids, input)) in &self.inputs {
			if !window_ids.contains(&window) {
				continue;
			}

			let input = input.borrow_mut();

			match delta {
				MouseScrollDelta::LineDelta(x, y) => {
					let scroll = if y > 0.0 {
						MouseScroll::Up
					} else if y < 0.0 {
						MouseScroll::Down
					} else if x > 0.0 {
						MouseScroll::Right
					} else if x < 0.0 {
						MouseScroll::Left
					} else {
						MouseScroll::None
					};

					input.mouse_scroll.emit(scroll);
				},
				_ => todo!()
			}
		}
	}
}
