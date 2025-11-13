use std::time;
use std::{any::{TypeId, type_name}, cell::{RefCell, RefMut}, path::{Path, PathBuf}, rc::Rc, sync::{Arc, Mutex, MutexGuard}};

use fatum_graphics::{Window, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::{PipelineKind, RenderTarget}};
use fatum_resources::{ResourcePlatform, Resources};

use crate::{Application, ApplicationInfo, GraphicsEngine, InputEngine, ResourceEngine, SceneEngine};

pub enum OutputKind {
	Window
}

pub struct CoreEngine<P: GraphicsPlatform + ResourcePlatform, A: Application<P>> {
	pub app: Box<A>,
	pub app_info: ApplicationInfo,
	pub base_directory: PathBuf,

	graphics: Rc<RefCell<GraphicsEngine<P>>>,
	resources: Arc<Mutex<ResourceEngine<P>>>,
	scene: Rc<RefCell<SceneEngine<P>>>,
	input: Rc<RefCell<InputEngine<P>>>,

	running: bool,

	last_loop: time::Instant,
	loop_delta: time::Duration,
}

impl<P, A> CoreEngine<P, A> where P: GraphicsPlatform + ResourcePlatform + Clone, A: Application<P> + Default {
	pub fn new(app: Box<A>) -> Self {
		// set up logging
		#[cfg(debug_assertions)]
		{
    		use ftail::Ftail;
   			use log::LevelFilter;

			Ftail::new()
				.formatted_console(LevelFilter::Debug)
				.init().unwrap();
		}

		#[cfg(not(debug_assertions))]
		{
			use ftail::Ftail;
   			use log::LevelFilter;

			Ftail::new()
				.console(LevelFilter::Info)
				.init().unwrap();
		}

		let base_directory = std::env::current_exe().map_or(
			Path::new(file!()).parent().unwrap().join(env!("CARGO_MANIFEST_DIR")),
			|p| {
				p.parent().unwrap().to_path_buf()
			}
		);

		log::info!("Base directory: {}", base_directory.display());

		let app_info = A::info();
		log::info!("Application: {:?}", app_info);

		let graphics = Rc::new(RefCell::new(GraphicsEngine::<P>::new(app_info.clone())));
		//                                                                                                this is AWESOME! --_\
		let resources = Arc::new(Mutex::new(ResourceEngine::<P>::new(Rc::new(graphics.clone().borrow_mut().get().clone()), &base_directory)));
		let scene = Rc::new(RefCell::new(SceneEngine::<P>::new(graphics.clone())));
		let input = Rc::new(RefCell::new(InputEngine::<P>::new(graphics.clone())));

		Self {
			app,
			app_info,
			base_directory,
			graphics,
			resources,
			scene,
			input,
			running: false,
			last_loop: time::Instant::now(),
			loop_delta: time::Duration::from_secs(0)
		}
	}

	pub fn graphics_engine(&mut self) -> RefMut<GraphicsEngine<P>> { self.graphics.borrow_mut() }
	pub fn resource_engine(&mut self) -> MutexGuard<ResourceEngine<P>> { self.resources.lock().unwrap() }
	pub fn scene_engine(&mut self) -> RefMut<SceneEngine<P>> { self.scene.borrow_mut() }
	pub fn input_engine(&mut self) -> RefMut<InputEngine<P>> { self.input.borrow_mut() }

	// pub fn graphics(&mut self) -> &mut P { self.graphics_engine().get() }
	// pub fn resources(&mut self) -> &mut Resources<P> { self.resource_engine().get() }

	pub fn setup(&mut self) {
		let mut app = std::mem::take(&mut self.app);
		app.setup(self);
		self.app = app;
	}

	pub fn run(&mut self) {
		self.running = true;

		while self.running {
			let now = time::Instant::now();
			let delta = now - self.last_loop;

			self.loop_delta = delta;
			self.last_loop = now;

			self.input_engine().process();

			{
				let mut app = std::mem::take(&mut self.app);
				app.process(self, delta);
				self.app = app;
			}

			self.scene_engine().process(delta);

			if !self.graphics_engine().process(delta) {
				self.running = false;
			}
		}
	}
}
