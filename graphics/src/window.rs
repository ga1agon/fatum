use glam::{UVec2, Vec2};

use crate::render::RenderTarget;

pub trait Window: RenderTarget {
	fn title(&self) -> &str;
	fn set_title(&mut self, title: &str);

	//fn size(&self) -> UVec2;
	//fn set_size(&mut self, size: UVec2);

	fn show(&mut self);
	fn hide(&mut self);
	fn close(self);

	fn should_close(&self) -> bool;
}
