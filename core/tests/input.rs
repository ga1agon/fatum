use std::{path::{Path, PathBuf}, str::FromStr};

use fatum::{Application, ApplicationInfo, CoreEngine, OutputKind, input::{ActionMap, InputAction, InputCombo, InputMap, Key}, resources::{ResText, ResTexture2D}};
use fatum_graphics::{Window, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::PipelineKind};
use fatum_resources::ResourcePlatform;

struct BasicApplication<P: GraphicsPlatform + ResourcePlatform> {
	_marker: std::marker::PhantomData<P>,
	input_map: Option<InputMap>
}

impl<P: GraphicsPlatform + ResourcePlatform + Clone> Application<P> for BasicApplication<P> {
	fn info() -> ApplicationInfo {
		ApplicationInfo {
			name: String::from("Basic Application")
		}
	}

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>) where Self: Sized {
		engine.graphics_engine().create_output(0, PipelineKind::Default, OutputKind::Window);

		let mut action_map = ActionMap::new(); // TODO resource

		// TODO does this need to be Rc<RefCell<>>?
		let action1 = InputAction::new("One");
		action_map.insert(vec![InputCombo::with_keys(vec![Key::A])], action1);

		self.input_map = engine.input_engine().create_input_map(0, action_map);
	}

	fn process(&mut self, engine: &mut CoreEngine<P, Self>, delta: std::time::Duration) where Self: Sized {
		
	}
}

impl<P: GraphicsPlatform + ResourcePlatform> Default for BasicApplication<P> {
	fn default() -> Self {
		Self {
			_marker: Default::default(),
			input_map: None
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
