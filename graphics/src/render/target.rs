use std::any::Any;

use glam::UVec2;

pub trait RenderTarget: Any {
	fn begin(&mut self);
	fn end(&mut self);

	fn size(&self) -> UVec2;

	fn is_active(&self) -> bool;
	fn set_active(&mut self, active: bool);

	fn as_any(&self) -> &dyn Any;
}
