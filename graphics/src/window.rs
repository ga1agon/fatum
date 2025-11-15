use glam::{UVec2, Vec2};
use winit::window::Window;

use crate::render::RenderTarget;

pub trait RenderWindow: RenderTarget {
	fn wimpl(&self) -> &winit::window::Window;
	fn wimpl_mut(&mut self) -> &mut winit::window::Window;

	fn title(&self) -> String { self.wimpl().title() }
	fn set_title(&self, title: &str) { self.wimpl().set_title(title); }

	fn show(&mut self) {
		self.wimpl().set_visible(true);
		self.set_active(true);
	}

	fn hide(&self) { self.wimpl().set_visible(false); }

	fn close(&mut self) where Self: Sized {
		self.wimpl().set_visible(false);
		self.set_active(false);
	}
}
