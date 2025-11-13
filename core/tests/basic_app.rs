use std::{path::{Path, PathBuf}, str::FromStr};

use fatum::{Application, ApplicationInfo, CoreEngine, OutputKind, resources::{ResText, ResTexture2D}};
use fatum_graphics::{Window, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::PipelineKind};
use fatum_resources::ResourcePlatform;

struct BasicApplication<P: GraphicsPlatform + ResourcePlatform> {
	_marker: std::marker::PhantomData<P>
}

impl<P: GraphicsPlatform + ResourcePlatform + Clone> Application<P> for BasicApplication<P> {
	fn info() -> ApplicationInfo {
		ApplicationInfo {
			name: String::from("Basic Application")
		}
	}

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>) where Self: Sized {
		engine.graphics_engine().create_output(0, PipelineKind::Default, OutputKind::Window);
		
		//let text = engine.resource_engine().get().load_by_path::<ResText>("hello.txt", false).unwrap();
		let text = engine.resource_engine().get().load_or_create::<ResText>(
			"hello.txt",
			ResText::new("meow meow meow!"),
			false
		).unwrap();

		log::info!("Text resource contents: {}", text.borrow().get());
	}
}

impl<P: GraphicsPlatform + ResourcePlatform> Default for BasicApplication<P> {
	fn default() -> Self {
		Self {
			_marker: Default::default()
		}
	}
}

#[test]
fn basic_application() {
	fatum::build::link_test_assets();

	let app = Box::new(BasicApplication::<OpenGlPlatform>::default());
	let mut engine = CoreEngine::<OpenGlPlatform, BasicApplication::<OpenGlPlatform>>::new(app);

	engine.setup();
	engine.run();
}
