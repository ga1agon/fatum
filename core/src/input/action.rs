use std::{cell::RefCell, rc::Rc};

use fatum_signals::StaticSignal;
use serde::{Deserialize, Serialize};

pub struct InputAction {
	name: String,

	pub(crate) up_state: bool,
	pub(crate) down_state: bool,
	pub(crate) pressed_state: bool,
	pub(crate) released_state: bool,

	pub up: StaticSignal<()>,
	pub down: StaticSignal<()>,
	pub pressed: StaticSignal<()>,
	pub released: StaticSignal<()>,
}

impl InputAction {
	pub fn new(name: &str) -> Rc<RefCell<InputAction>> {
		Rc::new(RefCell::new(Self {
			name: name.to_string(),
			up_state: false,
			down_state: false,
			pressed_state: false,
			released_state: false,
			up: StaticSignal::new(),
			down: StaticSignal::new(),
			pressed: StaticSignal::new(),
			released: StaticSignal::new(),
		}))
	}

	pub fn name(&self) -> &str { &self.name }

	pub fn is_up(&self) -> bool { self.up_state }
	pub fn is_down(&self) -> bool { self.down_state }
	pub fn was_pressed(&self) -> bool { self.pressed_state }
	pub fn was_released(&self) -> bool { self.released_state }
}
