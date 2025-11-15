use fatum_graphics::{Color, Material, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::RenderTarget};
use glam::UVec2;
use winit::{event::{Event, WindowEvent}, event_loop::{EventLoop, EventLoopBuilder}, platform::x11::EventLoopBuilderExtX11};
use std::{cell::RefCell, rc::Rc, *};

#[test]
fn opengl_open_window() {
	let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();

	let mut platform = OpenGlPlatform::new(&event_loop).unwrap();
	let mut window = platform.create_window(&event_loop, "Hello Window", UVec2::new(1280, 720))
		.unwrap();
	
	window.show();

	let mut queue = platform.create_queue();
	queue.add_target(window);

	queue.add_command(|delta| {
		println!("Delta time: {}", delta.as_secs_f32() * 1000.0);
	});

	let _ = event_loop.run(move |event: Event<()>, event_loop| {
		if let Event::WindowEvent { event, .. } = event {
			match event {
				WindowEvent::CloseRequested => {
					event_loop.exit();
				}
				WindowEvent::RedrawRequested => {
					queue.process();
				},
				_ => ()
			}
		}
	});
}
