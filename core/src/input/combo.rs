use std::hash::Hash;

use crate::input::{Key, MouseButton, MouseScrollWheel};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InputCombo {
	pub keys: Option<Vec<Key>>,
	pub mouse_buttons: Option<Vec<MouseButton>>,
	pub mouse_scroll_wheel: Option<MouseScrollWheel>,

	pub strict: bool
}

impl InputCombo {
	pub fn new(
		keys: Option<Vec<Key>>,
		mouse_buttons: Option<Vec<MouseButton>>,
		mouse_scroll_wheel: Option<MouseScrollWheel>
	) -> Self {
		let strict = if keys.as_ref().is_some_and(|v| v.len() > 1) { true } else { false };

		Self {
			keys,
			mouse_buttons,
			mouse_scroll_wheel,
			strict
		}
	}

	pub fn with_keys(keys: Vec<Key>) -> Self {
		Self::new(Some(keys), None, None)
	}

	pub fn with_mouse_buttons(mouse_buttons: Vec<MouseButton>) -> Self {
		Self::new(None, Some(mouse_buttons), None)
	}

	pub fn with_mouse_scroll_wheel(mouse_scroll_wheel: MouseScrollWheel) -> Self {
		Self::new(None, None, Some(mouse_scroll_wheel))
	}
}

impl Hash for InputCombo {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		if let Some(keys) = &self.keys {
			keys.hash(state);
		}

		if let Some(mouse_buttons) = &self.mouse_buttons {
			mouse_buttons.hash(state);
		}

		if let Some(mouse_scroll_wheel) = &self.mouse_scroll_wheel {
			mouse_scroll_wheel.hash(state);
		}

		self.strict.hash(state);
	}
}
