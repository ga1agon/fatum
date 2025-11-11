use fatum_scene::NodeBehaviour;
use fatum_signals::SignalDispatcher;

pub struct Updatable {
	pub signal_dispatcher: SignalDispatcher
}

impl Updatable {
	pub fn new() -> Self {
		Self {
			signal_dispatcher: SignalDispatcher::new()
		}
	}

	pub fn update(&self, delta: std::time::Duration) {
		self.signal_dispatcher.emit("update", delta);
	}
}

impl NodeBehaviour for Updatable {
	fn setup(&mut self) {
		self.signal_dispatcher.create_signal::<std::time::Duration>("update");
	}

	fn dispatcher(&self) -> &fatum_signals::SignalDispatcher { &self.signal_dispatcher }

	fn as_any(&self) -> &dyn std::any::Any { self }
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
