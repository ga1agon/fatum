use fatum_signals::{Signal, StaticSignal};

fn fn_ptr(args: &(u64, &str)) {
	println!("fn_ptr got: {}, {}", args.0, args.1);
}

#[test]
fn simple_test() {
	let mut signal: StaticSignal<(u64, &str)> = StaticSignal::new();

	let closure = |args: &(u64, &str)| {
		println!("closure got: {}, {}", args.0, args.1);
	};

	signal.connect(fn_ptr);
	signal.connect(closure);

	signal.emit((4, "hello"));
}
