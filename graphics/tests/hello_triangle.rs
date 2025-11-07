use fatum_graphics::{Color, Material, Mesh, Model, Vertex, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::{PipelineKind, RenderObject, RenderPipeline}};
use glam::{EulerRot, Mat4, Quat, UVec2, Vec3};
use std::{rc::Rc, *};

#[test]
fn opengl_hello_triangle() {
	let mut platform = OpenGlPlatform::new();
	let mut window = platform.create_window("Hello Triangle", UVec2::new(512, 512))
		.unwrap();

	window.show();

	let mut queue = platform.create_queue();
	queue.add_target(window);

	let pipeline = platform.create_pipeline(PipelineKind::Default);
	queue.set_pipeline(Some(pipeline));

	let triangle = Model {
		meshes: vec![
			Mesh {
				vertices: vec![
					Vertex::new(Vec3::new(-0.5, -0.5, 0.0), Default::default(), Default::default(), Default::default(), Default::default()),
					Vertex::new(Vec3::new( 0.5, -0.5, 0.0), Default::default(), Default::default(), Default::default(), Default::default()),
					Vertex::new(Vec3::new( 0.0,  0.5, 0.0), Default::default(), Default::default(), Default::default(), Default::default())
				],
				indices: vec![
					0u32, 1u32, 2u32
				],
				material: Material::with_color(Color::from_rgb_u8(255, 100, 150))
			}
		]
	};

	let translation = Vec3::new(0.0, 0.0, 0.0);
	let rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0);
	let scale = Vec3::new(1.0, 1.0, 1.0);
	let matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

	let triangle_object = Rc::new(RenderObject::new(triangle, matrix));
	queue.add_object(triangle_object);

	queue.add_command(|delta| {
		//println!("Delta time: {}", delta.as_secs_f32() * 1000.0);
	});

	loop {
		queue.process();
	}
}
