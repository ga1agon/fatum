use std::{cell::RefCell, path::{Path, PathBuf}, rc::Rc, str::FromStr};

use fatum::{Application, ApplicationInfo, CoreEngine, OutputKind, input::{ActionMap, InputAction, InputCombo, InputMap, MouseScroll}, resources::{ResActionMap, ResText, ResTexture2D}};
use fatum_graphics::{platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::PipelineKind};
use fatum_resources::ResourcePlatform;
use winit::{event::MouseButton, event_loop::{ControlFlow, EventLoop}, keyboard::KeyCode, platform::x11::EventLoopBuilderExtX11};

struct DefaultActionMap {}
impl DefaultActionMap {
	pub fn new() -> ActionMap {
		let mut action_map = ActionMap::new();

		// TODO does this need to be Rc<RefCell<>>?
		let action1 = InputAction::new("One");
		action_map.insert(vec![InputCombo::with_keys(vec![KeyCode::KeyA])], action1);

		let action2 = InputAction::new("Two");
		action_map.insert(vec![InputCombo::with_keys(vec![KeyCode::ControlLeft, KeyCode::KeyD])], action2);

		let action3 = InputAction::new("Three");
		action_map.insert(vec![InputCombo::with_mouse_buttons(vec![MouseButton::Left])], action3);

		let action4 = InputAction::new("Four");
		action_map.insert(vec![InputCombo::with_mouse_scroll_wheel(MouseScroll::Down)], action4);

		action_map
	}
}

struct BasicApplication<P: GraphicsPlatform + ResourcePlatform> {
	_marker: std::marker::PhantomData<P>,
	input_map: Option<Rc<RefCell<InputMap>>>
}

impl<P: GraphicsPlatform + ResourcePlatform + Clone> Application<P> for BasicApplication<P> {
	fn info() -> ApplicationInfo {
		ApplicationInfo {
			name: String::from("Basic Application")
		}
	}

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>, event_loop: &EventLoop<()>) where Self: Sized {
		engine.graphics_engine().create_queue(0, PipelineKind::Default);
		engine.graphics_engine().create_output(0, event_loop, OutputKind::Window);

		let action_map = engine.resource_engine().get().load_or_create(
			"input_test.actionmap",
			ResActionMap::new(DefaultActionMap::new()),
			true
		).unwrap();

		self.input_map = Some(engine.input_engine().create_input_map(0, action_map).expect("Couldn't create input map"));
	}

	fn process(&mut self, engine: &mut CoreEngine<P, Self>, delta: std::time::Duration) where Self: Sized {
		let input_map = self.input_map.clone().unwrap();
		let input_map = input_map.borrow();

		if input_map.was_action_pressed("One") {
			log::info!("One was pressed");
		}

		{
			let input = engine.input_engine().input(0).unwrap();
			let input = input.borrow();

			if input_map.is_action_down("Two") {
				log::info!("Cursor position: {}", input.cursor_position());
			}
		}

		if input_map.was_action_pressed("Three") {
			log::info!("Mouse button left was pressed");
		}

		if input_map.was_action_pressed("Four") {
			log::info!("Mouse scroll wheel down");
		}
	}
}

impl<P: GraphicsPlatform + ResourcePlatform> Default for BasicApplication<P> {
	fn default() -> Self {
		Self {
			_marker: Default::default(),
			input_map: Default::default()
		}
	}
}

#[test]
fn basic_application() {
	fatum::build::link_test_assets();

	let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();

	let app = Box::new(BasicApplication::<OpenGlPlatform>::default());
	let mut engine = CoreEngine::<OpenGlPlatform, BasicApplication::<OpenGlPlatform>>::new(app, &event_loop);

	engine.setup(&event_loop);
	event_loop.run_app(&mut engine).unwrap();
}
