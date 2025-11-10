use std::{any::Any, collections::HashMap};

use crate::{Signal, StaticSignal};

pub struct SignalDispatcher {
	signals: HashMap<String, Box<dyn Signal>>
}

impl SignalDispatcher {
	pub fn new() -> Self {
		Self {
			signals: HashMap::new()
		}
	}

	pub fn create_signal<Args: Copy + 'static>(&mut self, name: &str) {
		let signal = StaticSignal::<Args>::new();
		self.signals.insert(name.to_string(), Box::new(signal));
	}

	pub fn connect<Args: Copy + 'static, F: Fn(&Args) -> () + 'static>(&mut self, name: &str, handler: F) {
		let signal = self.signals.get_mut(&name.to_string())
			.expect(format!("No such signal: {}", name).as_str());

		// A Box Box !
		let handler = Box::new(Box::new(handler) as Box<dyn Fn(&Args)>) as Box<dyn Any>;
		signal.connect_any(handler);
	}

	pub fn disconnect<Args: Copy + 'static, F: Fn(&Args) -> () + 'static>(&mut self, name: &str, handler: F) {
		let signal = self.signals.get_mut(&name.to_string())
			.expect(format!("No such signal: {}", name).as_str());

		// Box Box <3
		let handler = Box::new(Box::new(handler) as Box<dyn Fn(&Args)>) as Box<dyn Any>;
		signal.disconnect_any(handler);
	}

	pub fn emit<Args: Copy + 'static>(&self, name: &str, args: Args) {
		let signal = self.signals.get(&name.to_string())
			.expect(format!("No such signal: {}", name).as_str());

		signal.emit_any(&args);
	}
}
