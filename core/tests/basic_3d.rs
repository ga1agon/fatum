use std::{path::{Path, PathBuf}, rc::Rc, str::FromStr};

use fatum::{Application, ApplicationInfo, CoreEngine, OutputKind, components::{Transform2D, Transform3D}, nodes::{Camera3D, Model3D}, resources::{ResText, ResTexture2D}};
use fatum_graphics::{Color, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::PipelineKind};
use fatum_resources::ResourcePlatform;
use fatum_scene::{Node, NodeId, SceneGraph};
use fatum_signals::SignalDispatcher;
use glam::{EulerRot, UVec2, Vec2, Vec3};
use winit::{event_loop::EventLoop, platform::x11::EventLoopBuilderExtX11};

struct Basic3DApplication<P: GraphicsPlatform + ResourcePlatform> {
	_marker: std::marker::PhantomData<P>
}

impl<P: GraphicsPlatform + ResourcePlatform + Clone> Application<P> for Basic3DApplication<P> {
	fn info() -> ApplicationInfo {
		ApplicationInfo {
			name: String::from("Basic 3D")
		}
	}

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>, event_loop: &EventLoop<()>) where Self: Sized {
		engine.graphics_engine().create_queue(0, PipelineKind::Default);
		engine.graphics_engine().create_output(0, event_loop, OutputKind::Window);

		let scene = SceneGraph::new();
		
		{
			let mut scene = scene.write().unwrap();

			let cube1: NodeId;
			{
				let mut model = fatum::nodes::UNIT_CUBE.clone();
				model.meshes[0].material.base_color = Color::from_rgb_u8(255, 0, 0);
				let model = Rc::new(Box::new(model));

				let mut node: Node = Model3D::new(model);
				node.connect_mut("$update", |args: &(*mut Node, std::time::Duration)| {
					let node = unsafe { &mut *args.0 };

					node.component_mut::<Transform3D>().unwrap()
						.rotate_euler(EulerRot::XZY, Vec3::new(0.0, 0.0, 1.0 * args.1.as_secs_f32()));
				});

				cube1 = scene.add_node(node, None);
			}

			let cube2: NodeId;
			{
				let mut model = fatum::nodes::UNIT_CUBE.clone();
				model.meshes[0].material.base_color = Color::from_rgb_u8(255, 0, 0);
				let model = Rc::new(Box::new(model));

				let mut node: Node = Model3D::new(model);
				node.component_mut::<Transform3D>().unwrap()
					.translate(Vec3::new(2.0, 1.0, 0.0));
				node.connect_mut("$update", |args: &(*mut Node, std::time::Duration)| {
					let node = unsafe { &mut *args.0 };

					node.component_mut::<Transform3D>().unwrap()
						.rotate_euler(EulerRot::XZY, Vec3::new(3.0 * args.1.as_secs_f32(), 0.0, 0.0));
				});

				cube2 = scene.add_node(node, Some(cube1));
			}

			let camera: NodeId;
			{
				let mut node = Camera3D::new_perspective(UVec2::new(1024, 768), 60.0, true);
				node.component_mut::<Transform3D>().unwrap()
					.translate(Vec3::new(2.0, 1.5, -3.0));
				node.component_mut::<Transform3D>().unwrap()
					.rotate_euler(EulerRot::YXZ, Vec3::new(-30.0f32.to_radians(), 15.0f32.to_radians(), 0.0));
				
				camera = scene.add_node(node.into(), None);
			}

			// scene.node_mut(camera).unwrap()
			// 	.component_mut::<Camera3D>().unwrap()
			// 	.set_target(Vec3::new(0.0, -0.5, 0.0));
		}

		engine.scene_engine().set_scene(0, scene);
	}
}

impl<P: GraphicsPlatform + ResourcePlatform> Default for Basic3DApplication<P> {
	fn default() -> Self {
		Self {
			_marker: Default::default()
		}
	}
}

#[test]
fn opengl_basic_3d() {
	fatum::build::link_test_assets();

	let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();

	let app = Box::new(Basic3DApplication::<OpenGlPlatform>::default());
	let mut engine = CoreEngine::<OpenGlPlatform, Basic3DApplication::<OpenGlPlatform>>::new(app, &event_loop);

	engine.setup(&event_loop);
	event_loop.run_app(&mut engine).unwrap();
}
