use std::{path::{Path, PathBuf}, str::FromStr};

use fatum::{Application, ApplicationInfo, CoreEngine, OutputKind, behaviours::Updatable, components::{Camera2D, Sprite2D, Transform2D}, resources::{ResText, ResTexture2D}};
use fatum_graphics::{Window, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::PipelineKind};
use fatum_resources::ResourcePlatform;
use fatum_scene::{SceneGraph, NodeBehaviour};
use glam::{UVec2, Vec2};

struct SceneGraphApplication<P: GraphicsPlatform + ResourcePlatform> {
	_marker: std::marker::PhantomData<P>
}

impl<P: GraphicsPlatform + ResourcePlatform + Clone> Application<P> for SceneGraphApplication<P> {
	fn info() -> ApplicationInfo {
		ApplicationInfo {
			name: String::from("Scene Graph")
		}
	}

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>) where Self: Sized {
		engine.graphics_engine().create_output(0, PipelineKind::Default, OutputKind::Window);
		let texture = engine.resource_engine().get().load_by_path::<ResTexture2D>("1.png", true).unwrap();

		let scene = SceneGraph::new();
		
		{
			let mut scene = scene.write().unwrap();

			// 1st sprite
			let mut sprite = Sprite2D::new_node(texture.clone());
			sprite.component_mut::<Transform2D>().unwrap()
				.set_translation(Vec2::new(100.0, 60.0)); // sprites have a pivot at their center
			sprite.component_mut::<Transform2D>().unwrap()
				.set_scale(Vec2::new(200.0, 120.0)); // 200x120 pixels
			
			let sprite = scene.add_node(sprite.into(), None);

			// 2nd sprite (parent of 1st)
			let mut sprite2 = Sprite2D::new_node(texture.clone());
			sprite2.component_mut::<Transform2D>().unwrap()
				.set_translation(Vec2::new(1024.0 / 2.0, 768.0 / 2.0)); // this won't be in the window's center, because it's a parent of sprite
			sprite2.component_mut::<Transform2D>().unwrap()
				.set_scale(Vec2::new(2.0, 3.0));

			let sprite2 = scene.add_node(sprite2, Some(sprite));

			// updatable
			let mut updatable = Updatable::new();
			scene.node_mut(sprite).unwrap()
				.add_behaviour(Box::new(updatable));

			scene.node_mut(sprite).unwrap()
				.behaviour_mut::<Updatable>().unwrap()
				.signal_dispatcher.connect("update", |delta: &std::time::Duration| {
					log::debug!("Update delta: {:?}", delta);
				});

			// camera
			let camera = Camera2D::new_node(UVec2::new(1024, 768), true);
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

	let app = Box::new(SceneGraphApplication::<OpenGlPlatform>::default());
	let mut engine = CoreEngine::<OpenGlPlatform, SceneGraphApplication::<OpenGlPlatform>>::new(app);

	engine.setup();
	engine.run();
}
