use std::any::Any;

pub trait RenderTarget: Any {
	fn begin(&mut self);
	fn end(&mut self);

	fn as_any(&self) -> &dyn Any;
}
