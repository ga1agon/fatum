use std::{any::Any, sync::{Arc, RwLock, RwLockReadGuard}};

pub trait Signal {
	/// This function is DANGEROUS and should be used with CAUTION!!!
	fn connect_any(&mut self, handler: Box<dyn Any>);

	/// This function is DANGEROUS and should be used with CAUTION!!!
	fn disconnect_any(&mut self, handler: Box<dyn Any>);

	/// This function is DANGEROUS and should be used with CAUTION!!!
	fn emit_any(&self, args: &dyn Any);

	fn clear(&mut self);
}

pub struct StaticSignal<Args: Copy + 'static> {
	handlers: Vec<Box<dyn Fn(&Args) -> ()>>
}

impl<Args> StaticSignal<Args> where Args: Copy + 'static {
	pub fn new() -> Self {
		Self {
			handlers: Vec::new()
		}
	}

	pub fn connect<F: Fn(&Args) -> () + 'static>(&mut self, handler: F) {
		self.handlers.push(Box::new(handler));
	}

	pub fn disconnect<F: Fn(&Args) -> () + 'static>(&mut self, handler: F) {
		let mut index: Option<usize> = None;

		for i in 0..self.handlers.len() {
			let e_handler = &self.handlers[i];
			// derefdereferencing a &Box<T> into T waowww
			if (**e_handler).type_id() == handler.type_id() {
				index = Some(i);
				break;
			}
		}

		if index.is_none() {
			log::warn!("Not removing handler {:?}: doesn't exist in the current signal", handler.type_id());
		}

		self.handlers.remove(index.unwrap());
	}

	pub fn emit(&self, args: Args) {
		for handler in &self.handlers {
			handler.as_ref().call((&args,));
		}
	}
}

impl<Args> Signal for StaticSignal<Args> where Args: Copy + 'static {
	fn connect_any(&mut self, handler: Box<dyn Any>) {
		let handler_type_name = std::any::type_name_of_val(&handler);

		// Box !
		let handler = *handler.downcast::<Box<dyn Fn(&Args) -> ()>>()
			.expect(format!(
				"Cannot connect handler - invalid type ({} required, got {} instead)",
				std::any::type_name::<Box<dyn Fn(&Args) -> ()>>(),
				handler_type_name
			).as_str());

		self.handlers.push(handler);
	}

	fn disconnect_any(&mut self, handler: Box<dyn Any>) {
		let handler = *handler.downcast::<Box<dyn Fn(&Args) -> ()>>()
			.expect("Cannot disconnect handler - invalid type");

		let mut index: Option<usize> = None;

		for i in 0..self.handlers.len() {
			let e_handler = &self.handlers[i];
			// derefdereferencing a &Box<T> into T waowww
			if (**e_handler).type_id() == (*handler).type_id() {
				index = Some(i);
				break;
			}
		}

		if index.is_none() {
			log::warn!("Not removing handler {:?}: doesn't exist in the current signal", handler.type_id());
		}

		self.handlers.remove(index.unwrap());
	}

	fn emit_any(&self, args: &dyn Any) {
		let args = args.downcast_ref::<Args>()
			.expect("Signal called with invalid arguments");
		
		for handler in &self.handlers {
			handler.as_ref().call((args,));
		}
	}

	fn clear(&mut self) {
		self.handlers.clear();
	}
}

// pub struct DynamicSignal {
// 	handlers: Vec<Box<dyn Fn(&dyn Any) -> ()>>
// }

// impl DynamicSignal {
// 	pub fn new() -> Self {
// 		Self {
// 			handlers: Vec::new()
// 		}
// 	}

// 	pub fn connect<F: Fn(&dyn Any) -> ()>(&mut self, handler: F) {
// 		self.handlers.push(Box::new(handler));
// 	}

// 	fn emit(&self, args: &dyn Any) {
// 		for handler in &self.handlers {
// 			handler.as_ref().call((args,));
// 		}
// 	}
// }

// impl<Args> Signal for StaticSignal<Args> where Args: Copy + 'static {
// 	fn emit(&self, args: &dyn Any) {
// 		let args = args.downcast_ref::<Args>()
// 			.expect("Signal called with the wrong arguments");

// 		for handler in &self.handlers {
// 			handler.as_ref().call((args,));
// 		}
// 	}
// }
