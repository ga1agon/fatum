use fatum_signals::{Signal, SignalDispatcher, StaticSignal};

fn fn_ptr(args: &(u64, &str)) {
	println!("fn_ptr got: {}, {}", args.0, args.1);
}

#[test]
fn simple_test() {
	let meow_closure = |args: &(u64, &str)| {
		println!("meow got: {}, {}", args.0, args.1);
	};

	let awoo_closure = |args: &bool| {
		println!("awoo got: {}", args);
	};

	let mut dispatcher = SignalDispatcher::new();
	dispatcher.create_signal::<(u64, &str)>("meow");
	dispatcher.create_signal::<bool>("awoo");

	dispatcher.connect("meow", fn_ptr);
	dispatcher.connect("meow", meow_closure);
	dispatcher.connect("awoo", awoo_closure);

	dispatcher.emit("meow", (4u64, "hello"));
	dispatcher.emit("awoo", true);
}
