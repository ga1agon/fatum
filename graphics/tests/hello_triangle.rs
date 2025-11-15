use fatum_graphics::{Camera2D, Color, Material, Mesh, Model, Vertex, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::{PipelineKind, RenderObject, RenderPipeline}};
use glam::{EulerRot, Mat4, Quat, UVec2, Vec2, Vec3};
use winit::{event::{Event, WindowEvent}, event_loop::EventLoop, platform::x11::EventLoopBuilderExtX11};
use std::{rc::Rc, *};

#[test]
fn opengl_hello_triangle() {
	let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
	let mut platform = OpenGlPlatform::new(&event_loop).unwrap();
	let mut window = platform.create_window(&event_loop, "Hello Triangle", UVec2::new(800, 600))
		.unwrap();
	
	window.show();
	window.begin();

	let mut queue = platform.create_queue();
	let window = queue.add_target(window);

	let pipeline = platform.create_pipeline(PipelineKind::Default);
	queue.set_pipeline(Some(pipeline));

	let triangle = Rc::new(Box::new(Model {
		meshes: vec![
			Mesh {
				vertices: vec![
					Vertex::new(Vec3::new(200.0, 400.0, 0.0), Default::default(), Default::default(), Default::default(), Default::default()),
					Vertex::new(Vec3::new(0.0, 0.0, 0.0), Default::default(), Default::default(), Default::default(), Default::default()),
					Vertex::new(Vec3::new(400.0, 0.0, 0.0), Default::default(), Default::default(), Default::default(), Default::default())
				],
				indices: vec![
					0u32, 1u32, 2u32
				],
				material: Material::with_color(Color::from_rgb_u8(255, 100, 150))
			}
		]
	}));

	let translation = Vec3::new(0.0, 0.0, 0.0);
	let rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0);
	let scale = Vec3::new(1.0, 1.0, 1.0);
	let matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

	let triangle_object = RenderObject::new(triangle.clone());
	queue.add_object(&triangle_object, matrix);

	let camera = Camera2D {
		position: Vec2::new(-200.0, -50.0),
		up: Camera2D::UP,
		size: queue.get_target(window).unwrap().size()
	};

	queue.pipeline_mut().unwrap().camera_data().set_data(vec![camera.create()].into());

	queue.add_command(|delta| {
		//println!("Delta time: {}", delta.as_secs_f32() * 1000.0);
	});

	// while queue.is_active() {
	// 	queue.process();
	// }
	let _ = event_loop.run(move |event: Event<()>, event_loop| {
		if let Event::WindowEvent { event, .. } = event {
			match event {
				WindowEvent::CloseRequested => {
					event_loop.exit();
				}
				WindowEvent::RedrawRequested => {
					queue.process();
				}
				_ => (),
			}
		}
	});
}
