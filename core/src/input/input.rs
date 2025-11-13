use fatum_signals::StaticSignal;
use glam::Vec2;
use glfw::{Action, Modifiers};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::input::{Key, MouseButton};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Hash, Default)]
pub enum CursorMode {
	#[default]
	Normal = glfw::ffi::GLFW_CURSOR_NORMAL,
	Hidden = glfw::ffi::GLFW_CURSOR_HIDDEN,
	Disabled = glfw::ffi::GLFW_CURSOR_DISABLED,
}

// TODO scroll & move move (mouse_position() fn)
pub struct Input {
	pub key_up: StaticSignal<(Key, Modifiers)>,
	pub key_down: StaticSignal<(Key, Modifiers)>,
	pub key_repeat: StaticSignal<(Key, Modifiers)>,
	pub mouse_button_up: StaticSignal<(MouseButton, Modifiers)>,
	pub mouse_button_down: StaticSignal<(MouseButton, Modifiers)>,

	pub(crate) cursor_position: Vec2,
	pub(crate) cursor_mode: CursorMode,

	pub(crate) cursor_mode_set: StaticSignal<CursorMode>,
}

impl Input {
	pub fn new() -> Self {
		Self {
			key_up: StaticSignal::new(),
			key_down: StaticSignal::new(),
			key_repeat: StaticSignal::new(),
			mouse_button_up: StaticSignal::new(),
			mouse_button_down: StaticSignal::new(),
			cursor_position: Vec2::ZERO,
			cursor_mode: CursorMode::Normal,
			cursor_mode_set: StaticSignal::new()
		}
	}

	pub fn cursor_position(&self) -> Vec2 { self.cursor_position }

	pub fn cursor_mode(&self) -> CursorMode { self.cursor_mode }
	pub fn set_cursor_mode(&mut self, mode: CursorMode) {
		self.cursor_mode = mode;
		self.cursor_mode_set.emit(mode);
	}
}

impl Default for Input {
	fn default() -> Self {
		Self::new()
	}
}
