use fatum_signals::StaticSignal;
use glfw::{Action, Modifiers};

use crate::input::{Key, MouseButton};

// TODO scroll & move move (mouse_position() fn)
pub struct Input {
	pub key_up: StaticSignal<(Key, Modifiers)>,
	pub key_down: StaticSignal<(Key, Modifiers)>,
	pub key_repeat: StaticSignal<(Key, Modifiers)>,
	pub mouse_button_up: StaticSignal<(MouseButton, Modifiers)>,
	pub mouse_button_down: StaticSignal<(MouseButton, Modifiers)>
}

impl Input {
	pub fn new() -> Self {
		Self {
			key_up: StaticSignal::new(),
			key_down: StaticSignal::new(),
			key_repeat: StaticSignal::new(),
			mouse_button_up: StaticSignal::new(),
			mouse_button_down: StaticSignal::new()
		}
	}
}
