use fatum_graphics::{Camera2D, Color, Material, Mesh, Model, Vertex, Window, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::{PipelineKind, RenderObject, RenderPipeline}, texture};
use glam::{EulerRot, Mat4, Quat, UVec2, Vec2, Vec3};
use std::{fs::File, path::Path, rc::Rc, *};

#[test]
fn opengl_textures() {
	let mut platform = OpenGlPlatform::new();
	let mut window = platform.create_window("Textures", UVec2::new(800, 600))
		.unwrap();

	window.show();

	let mut queue = platform.create_queue();
	let window = queue.add_target(window);

	let pipeline = platform.create_pipeline(PipelineKind::Default);
	queue.set_pipeline(Some(pipeline));

	let texture_image_path = Path::new(file!()).parent().unwrap().join(env!("CARGO_MANIFEST_DIR")).join("tests/trollface.png");
	let texture_image = image::open(texture_image_path).unwrap();
	let texture = platform.create_texture_2d(texture_image, texture::Options::default()).unwrap();

	let square = Rc::new(Box::new(Model {
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
				material: Material::with_textures_pbr(
					Color::from_rgb_u8(255, 255, 255),
					0.5,
					0.5,
					1.5,
					Some(&texture),
					None,
					None,
					None,
					None
				)
			}
		]
	}));

	let translation = Vec3::new(0.0, 0.0, 0.0);
	let rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0);
	let scale = Vec3::new(1.0, 1.0, 1.0);
	let matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

	let square_object = RenderObject::new(square.clone());
	queue.add_object(&square_object, matrix);

	let camera = Camera2D {
		position: Vec2::ZERO,
		up: Camera2D::UP,
		size: queue.get_target(window).unwrap().size()
	};

	queue.pipeline_mut().unwrap().camera_data().set_data(vec![camera.create()].into());

	while queue.is_active() {
		queue.process();
	}
}
