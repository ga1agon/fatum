use fatum_signals::SignalDispatcher;

pub trait NodeBehaviour {
	fn setup(&mut self);
	fn dispatcher(&self) -> &SignalDispatcher;

	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
