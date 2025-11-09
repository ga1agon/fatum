use std::{any::{TypeId, type_name}, path::{Path, PathBuf}, rc::Rc, sync::{Arc, Mutex}};

use fatum_graphics::{Window, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::{PipelineKind, RenderTarget}};
use fatum_resources::{ResourcePlatform, Resources};

use crate::{Application, ApplicationInfo, GraphicsEngine, ResourceEngine};

pub enum OutputKind {
	Window
}

pub struct CoreEngine<P: GraphicsPlatform + ResourcePlatform, A: Application<P>> {
	pub app: Box<A>,
	pub app_info: ApplicationInfo,
	pub base_directory: PathBuf,
	graphics: GraphicsEngine<P>,
	resources: ResourceEngine<P>,

	running: bool
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

		let mut graphics = GraphicsEngine::<P>::new(app_info.clone());
		let resources = ResourceEngine::<P>::new(graphics.get(), &base_directory);

		Self {
			app,
			app_info,
			base_directory,
			graphics,
			resources,
			running: false
		}
	}

	pub fn graphics_engine(&mut self) -> &mut GraphicsEngine<P> { &mut self.graphics }
	pub fn resource_engine(&mut self) -> &mut ResourceEngine<P> { &mut self.resources }

	pub fn graphics(&mut self) -> &mut P { self.graphics.get() }
	pub fn resources(&mut self) -> &mut Resources<P> { self.resources.get() }

	pub fn setup(&mut self) {
		let mut app = std::mem::take(&mut self.app);
		app.setup(self);
		self.app = app;
	}

	pub fn run(&mut self) {
		self.running = true;

		while self.running {
			if !self.graphics.process() {
				self.running = false;
			}
		}
	}
}
