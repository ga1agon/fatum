use fatum::resources::{MetaTexture2D, ResTexture2D};
use fatum_graphics::{Camera2D, Color, Material, Mesh, Model, Vertex, Window, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::{PipelineKind, RenderObject, RenderPipeline}, texture};
use fatum_resources::Resources;
use glam::{EulerRot, Mat4, Quat, UVec2, Vec2, Vec3};
use simple_logger::SimpleLogger;
use std::{fs::File, path::Path, rc::Rc, *};

#[test]
fn opengl_scene_graph() {
	SimpleLogger::new().init().unwrap();

	let mut platform = OpenGlPlatform::new();
	let mut window = platform.create_window("Scene Graph", UVec2::new(800, 800))
		.unwrap();

	window.show();

	let mut queue = platform.create_queue();
	let window = queue.add_target(window);

	let pipeline = platform.create_pipeline(PipelineKind::Default);
	queue.set_pipeline(Some(pipeline));

	let mut resources = Resources::new(platform.clone().into(), Path::new(file!()).parent().unwrap().join(env!("CARGO_MANIFEST_DIR")).join("tests/assets"));
	let texture = resources.load_by_path::<ResTexture2D, MetaTexture2D, &str>("1.png", false).unwrap();

	let square1 = Model {
		meshes: vec![
			Mesh {
				vertices: vec![
					Vertex::new(Vec3::new(0.0, 0.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(0.0, 0.0)),
					Vertex::new(Vec3::new(0.0, 200.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(0.0, 1.0)),
					Vertex::new(Vec3::new(200.0, 200.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(1.0, 1.0)),
					Vertex::new(Vec3::new(200.0, 0.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(1.0, 0.0)),
				],
				indices: vec![
					0u32, 1u32, 2u32,
					0u32, 2u32, 3u32
				],
				material: Material::with_color(Color::from_rgb_u8(255, 255, 255))
			}
		]
	};

	// let translation = Vec3::new(0.0, 0.0, 0.0);
	// let rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0);
	// let scale = Vec3::new(1.0, 1.0, 1.0);
	// let matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

	// let square_object = Rc::new(RenderObject::new(square, matrix));
	// queue.add_object(square_object);

	let camera = Camera2D {
		position: Vec2::ZERO,
		size: queue.get_target(window).unwrap().size()
	};

	queue.pipeline_mut().unwrap().camera_data().set_data(vec![camera.create()].into());

	while queue.is_active() {
		queue.process();
	}
}
