use std::{path::{Path, PathBuf}, str::FromStr};

use fatum::{Application, ApplicationInfo, CoreEngine, OutputKind, components::Transform2D, nodes::{Camera2D, Sprite2D}, resources::{ResText, ResTexture2D}};
use fatum_graphics::{platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::PipelineKind};
use fatum_resources::ResourcePlatform;
use fatum_scene::{Node, SceneGraph};
use fatum_signals::SignalDispatcher;
use glam::{UVec2, Vec2};
use winit::{event_loop::EventLoop, platform::x11::EventLoopBuilderExtX11};

struct SceneGraphApplication<P: GraphicsPlatform + ResourcePlatform> {
	_marker: std::marker::PhantomData<P>
}

impl<P: GraphicsPlatform + ResourcePlatform + Clone> Application<P> for SceneGraphApplication<P> {
	fn info() -> ApplicationInfo {
		ApplicationInfo {
			name: String::from("Scene Graph")
		}
	}

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>, event_loop: &EventLoop<()>) where Self: Sized {
		engine.graphics_engine().create_queue(0, PipelineKind::Default);
		engine.graphics_engine().create_output(0, &event_loop, OutputKind::Window);

		let texture = engine.resource_engine().get().load_by_path::<ResTexture2D>("1.png", true).unwrap();

		let scene = SceneGraph::new();
		
		{
			let mut scene = scene.write().unwrap();

			// 1st sprite
			let mut sprite = Sprite2D::new(texture.clone());
			sprite.component_mut::<Transform2D>().unwrap()
				.set_translation(Vec2::new(100.0, 60.0)); // sprites have a pivot at their center
			sprite.component_mut::<Transform2D>().unwrap()
				.set_scale(Vec2::new(200.0, 120.0)); // 200x120 pixels
			
			let sprite = scene.add_node(sprite.into(), None);

			// 2nd sprite (parent of 1st)
			// TODO how to fix transform being scaled up by the parent's scale?? is it a 2D only issue?
			let mut sprite2 = Sprite2D::new(texture.clone());
			sprite2.component_mut::<Transform2D>().unwrap()
				.set_translation(Vec2::new(4.0, 3.0)); // this won't be in the window's center, because it's a parent of sprite
			sprite2.component_mut::<Transform2D>().unwrap()
				.set_scale(Vec2::new(2.0, 3.0));

			let sprite2 = scene.add_node(sprite2, Some(sprite));

			// signaling
			{
				let sprite = scene.node_mut(sprite).unwrap();

				sprite.connect("ready", |args: &(*const Node, ())| {
					log::info!("sprite2 is ready!");
				});

				sprite.connect_mut("$update", |args: &(*mut Node, std::time::Duration)| {
					let node = unsafe { &mut *args.0 };

					node.component_mut::<Transform2D>().unwrap()
						.rotate(2.0 * args.1.as_secs_f32());
				});
			}

			// camera
			let camera = Camera2D::new(UVec2::new(1024, 768), true);
			scene.add_node(camera.into(), None);
		}

		engine.scene_engine().set_scene(0, scene);
	}
}

impl<P: GraphicsPlatform + ResourcePlatform> Default for SceneGraphApplication<P> {
	fn default() -> Self {
		Self {
			_marker: Default::default()
		}
	}
}

#[test]
fn opengl_scene_graph() {
	fatum::build::link_test_assets();

	let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();

	let app = Box::new(SceneGraphApplication::<OpenGlPlatform>::default());
	let mut engine = CoreEngine::<OpenGlPlatform, SceneGraphApplication::<OpenGlPlatform>>::new(app, &event_loop);

	engine.setup(&event_loop);
	event_loop.run_app(&mut engine).unwrap();
}
