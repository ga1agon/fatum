use fatum_graphics::{Color, Material, platform::{GraphicsPlatform, opengl::OpenGlPlatform}};
use glam::UVec2;
use std::*;

#[test]
fn opengl_open_window() {
	let mut platform = OpenGlPlatform::new();
	let mut window = platform.create_window("Hello Window", UVec2::new(1280, 720))
		.unwrap();

	window.show();

	let mut queue = platform.create_queue();
	queue.add_target(window);

	queue.add_command(|delta| {
		println!("Delta time: {}", delta.as_secs_f32() * 1000.0);
	});

	loop {
		queue.process();
	}

	//thread::sleep(time::Duration::from_secs(2));
	//window.close();
}
