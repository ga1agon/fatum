pub trait NodeComponent: 'static {
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> NodeComponent for T {
	fn as_any(&self) -> &dyn std::any::Any { self }
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
