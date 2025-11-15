use fatum_signals::StaticSignal;
use glam::Vec2;
use winit::{event::MouseButton, keyboard::{Key, KeyCode}, window::CursorGrabMode};

use crate::input::MouseScroll;

pub struct Input {
	pub key_up: StaticSignal<KeyCode>,
	pub key_down: StaticSignal<KeyCode>,
	pub mouse_button_up: StaticSignal<MouseButton>,
	pub mouse_button_down: StaticSignal<MouseButton>,
	pub mouse_scroll: StaticSignal<MouseScroll>,

	pub(crate) cursor_position: Vec2,
	pub(crate) cursor_mode: CursorGrabMode,

	pub(crate) cursor_mode_set: StaticSignal<CursorGrabMode>,
}

impl Input {
	pub fn new() -> Self {
		Self {
			key_up: StaticSignal::new(),
			key_down: StaticSignal::new(),
			mouse_button_up: StaticSignal::new(),
			mouse_button_down: StaticSignal::new(),
			mouse_scroll: StaticSignal::new(),
			cursor_position: Vec2::ZERO,
			cursor_mode: CursorGrabMode::None,
			cursor_mode_set: StaticSignal::new()
		}
	}

	pub fn cursor_position(&self) -> Vec2 { self.cursor_position }

	pub fn cursor_mode(&self) -> CursorGrabMode { self.cursor_mode }
	pub fn set_cursor_mode(&mut self, mode: CursorGrabMode) {
		self.cursor_mode = mode;
		self.cursor_mode_set.emit(mode);
	}
}

impl Default for Input {
	fn default() -> Self {
		Self::new()
	}
}
