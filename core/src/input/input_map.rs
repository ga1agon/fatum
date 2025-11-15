use std::{cell::{Ref, RefCell}, collections::HashMap, fmt::Debug, rc::Rc};

use fatum_resources::ResourceRef;
use winit::{event::MouseButton, keyboard::KeyCode};

use crate::{input::{ActionMap, Input, InputAction, MouseScroll}, resources::ResActionMap};

#[derive(Clone)]
pub struct InputMap {
	input: Rc<RefCell<Input>>,
	action_map: ResourceRef<ResActionMap>,
	actions: HashMap<String, Rc<RefCell<InputAction>>>,

	current_key_combo: Rc<RefCell<Vec<KeyCode>>>,
	current_mouse_button_combo: Rc<RefCell<Vec<MouseButton>>>,
	current_scroll_wheel: Rc<RefCell<MouseScroll>>
}

impl InputMap {
	pub fn new(input: Rc<RefCell<Input>>, action_map: ResourceRef<ResActionMap>) -> Self {
		let mut actions = HashMap::new();

		for (_, action) in action_map.borrow().get() {
			let name: String;

			{
				let action = action.borrow();
				name = action.name().to_string();
			}

			actions.insert(name, action.clone());
		}

		let this = Self {
			input,
			action_map,
			actions,
			current_key_combo: Rc::new(RefCell::new(Vec::new())),
			current_mouse_button_combo: Rc::new(RefCell::new(Vec::new())),
			current_scroll_wheel: Rc::new(RefCell::new(MouseScroll::None))
		};

		// warning: extremely ugly
		let ckc1 = this.current_key_combo.clone();
		this.input.borrow_mut().key_up.connect(move |args| {
			ckc1.borrow_mut().retain(|v| *v != *args);
		});

		let ckc2 = this.current_key_combo.clone();
		this.input.borrow_mut().key_down.connect(move |args| {
			ckc2.borrow_mut().push(*args);
		});

		let cmbc1 = this.current_mouse_button_combo.clone();
		this.input.borrow_mut().mouse_button_up.connect(move |args| {
			cmbc1.borrow_mut().retain(|v| *v != *args);
		});

		let cmbc2 = this.current_mouse_button_combo.clone();
		this.input.borrow_mut().mouse_button_down.connect(move |args| {
			cmbc2.borrow_mut().push(*args);
		});

		let csw1 = this.current_scroll_wheel.clone();
		this.input.borrow_mut().mouse_scroll.connect(move |args| {
			*csw1.borrow_mut() = *args;
		});

		this
	}

	pub fn process(&mut self) {
		for (combos, action) in self.action_map.borrow_mut().get_mut() {
			let action = action.clone();
			let mut action = action.borrow_mut();
			
			action.pressed_state = false;
			action.released_state = false;

			let mut any_combo_down = false;

			for combo in combos {
				if let Some(combo_keys) = &combo.keys {
					let ckc = self.current_key_combo.borrow();

					let condition =
						ckc.iter().eq(combo_keys)
						|| (!combo.strict && combo_keys.iter().all(|v| { ckc.contains(v) }));

					if condition {
						any_combo_down = true;
						break;
					}
				}

				if let Some(combo_mouse_buttons) = &combo.mouse_buttons {
					let cmbc = self.current_mouse_button_combo.borrow();

					let condition =
						cmbc.iter().eq(combo_mouse_buttons)
						|| (!combo.strict && combo_mouse_buttons.iter().all(|v| { cmbc.contains(v) }));

					if condition {
						any_combo_down = true;
						break;
					}
				}

				if let Some(combo_scroll_wheel) = &combo.mouse_scroll_wheel {
					let csw = self.current_scroll_wheel.borrow();

					if *combo_scroll_wheel == *csw {
						any_combo_down = true;
						break;
					}
				}
			}

			if any_combo_down {
				if !action.down_state {
					action.down_state = true;
					action.up_state = false;
					action.pressed_state = true;
					action.pressed.emit(());
				}

				action.down.emit(());
			} else {
				if action.down_state {
					action.down_state = false;
					action.up_state = true;
					action.released_state = true;
					action.released.emit(());
				}

				action.up.emit(());
			}
		}

		*self.current_scroll_wheel.borrow_mut() = MouseScroll::None;
	}

	pub fn action(&self, name: &str) -> Option<Rc<RefCell<InputAction>>> {
		self.actions.get(name).cloned()
	}

	pub fn is_action_up(&self, action: &str) -> bool {
		if let Some(action) = self.action(action) {
			let action = action.borrow();
			return action.is_up();
		}

		false
	}

	pub fn is_action_down(&self, action: &str) -> bool {
		if let Some(action) = self.action(action) {
			let action = action.borrow();
			return action.is_down();
		}

		false
	}

	pub fn was_action_pressed(&self, action: &str) -> bool {
		if let Some(action) = self.action(action) {
			let action = action.borrow();
			return action.was_pressed();
		}

		false
	}

	pub fn was_action_released(&self, action: &str) -> bool {
		if let Some(action) = self.action(action) {
			let action = action.borrow();
			return action.was_released();
		}

		false
	}
}

