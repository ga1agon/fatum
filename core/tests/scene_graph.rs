use std::{path::{Path, PathBuf}, str::FromStr};

use fatum::{Application, ApplicationInfo, CoreEngine, OutputKind, nodes::Sprite2D, resources::{ResText, ResTexture2D}};
use fatum_graphics::{Window, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::PipelineKind};
use fatum_resources::ResourcePlatform;
use fatum_scene::SceneTree;

struct SceneGraphApplication<P: GraphicsPlatform + ResourcePlatform> {
	_marker: std::marker::PhantomData<P>
}

impl<P: GraphicsPlatform + ResourcePlatform + Clone> Application<P> for SceneGraphApplication<P> {
	fn info() -> ApplicationInfo {
		ApplicationInfo {
			name: String::from("Basic Application")
		}
	}

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>) where Self: Sized {
		engine.graphics_engine().create_output(0, PipelineKind::Default, OutputKind::Window);
		let texture = engine.resource_engine().get().load_by_path::<ResTexture2D>("1.png", true).unwrap();

		let scene = SceneTree::new();
		
		{
			let mut scene = scene.lock().unwrap();
			let sprite = Sprite2D::new(texture.clone());
			
			scene.add_node(sprite.into(), None);
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
