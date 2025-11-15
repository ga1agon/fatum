use std::time;
use std::{any::{TypeId, type_name}, cell::{RefCell, RefMut}, path::{Path, PathBuf}, rc::Rc, sync::{Arc, Mutex, MutexGuard}};

use fatum_graphics::RenderWindow;
use fatum_graphics::platform::opengl::OpenGlWindow;
use fatum_graphics::{platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::{PipelineKind, RenderTarget}};
use fatum_resources::{ResourcePlatform, Resources};
use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::platform::x11::EventLoopBuilderExtX11;

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

	pub running: bool,

	last_loop: time::Instant,
	loop_delta: time::Duration,
}

impl<P, A> CoreEngine<P, A> where P: GraphicsPlatform + ResourcePlatform + Clone, A: Application<P> + Default {
	pub fn new(app: Box<A>, event_loop: &EventLoop<()>) -> Self {
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

		//let event_loop = Rc::new(EventLoop::builder().with_any_thread(true).build().unwrap());

		let graphics = Rc::new(RefCell::new(GraphicsEngine::<P>::new(&event_loop, app_info.clone())));
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

	pub fn setup(&mut self, event_loop: &EventLoop<()>) {
		let mut app = std::mem::take(&mut self.app);
		app.setup(self, event_loop);
		self.app = app;
	}

	// pub fn run(mut self, event_loop: &EventLoop<()>) {
	// 	self.running = true;

	// 	// this is FUCKING AWESOME
	// 	// let event_loop = Rc::clone(&self.event_loop);
	// 	// //let event_loop = std::mem::replace(&mut self.event_loop, unsafe { std::mem::zeroed() });
	// 	// event_loop.run_app(&mut self);
	// 	//self.event_loop = event_loop;

	// 	event_loop.run_app(&mut self);
	// }
}

impl<P, A> ApplicationHandler<()> for CoreEngine<P, A> where P: GraphicsPlatform + ResourcePlatform + Clone, A: Application<P> + Default {
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {}

	// we love ten billion for loops
	fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
		let mut graphics = self.graphics_engine();

		for (_, queue) in graphics.outputs() {
			for target in queue.targets() {
				let target = queue.get_target_mut(target).unwrap();
				
				// ideally we could get a dyn Window but fuck you kind of
				if let Some(window_target) = target.as_any_mut().downcast_mut::<OpenGlWindow>() {
					window_target.wimpl().request_redraw();
				}
			}
		}
	}

	fn window_event(
		&mut self,
		event_loop: &winit::event_loop::ActiveEventLoop,
		window_id: winit::window::WindowId,
		event: WindowEvent,
	) {
		match event {
			WindowEvent::CloseRequested => {
				{
					let mut graphics = self.graphics_engine();

					for (_, queue) in graphics.outputs() {
						for target in queue.targets() {
							let target = queue.get_target_mut(target).unwrap();
							
							// ideally we could get a dyn Window but fuck you kind of
							if let Some(window_target) = target.as_any_mut().downcast_mut::<OpenGlWindow>() {
								window_target.close();
							} else {
								target.set_active(false);
							}
						}
					}
				}

				if !self.graphics_engine().is_active() {
					self.running = false;
					event_loop.exit();
				}
			},
			// TODO this should (obviously) redraw only one window but that would probably mean rewriting everything
			WindowEvent::RedrawRequested => {
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
				self.graphics_engine().process(delta);

				if !self.graphics_engine().is_active() {
					self.running = false;
					event_loop.exit();
				}
			},
			WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
				if event.repeat { return; }
				self.input_engine().on_keyboard_input(window_id, device_id, event);
			},
			WindowEvent::CursorMoved { device_id, position } => {
				self.input_engine().on_mouse_move(window_id, device_id, position);
			},
			WindowEvent::MouseInput { device_id, state, button } => {
				self.input_engine().on_mouse_input(window_id, device_id, button, state);
			},
			WindowEvent::MouseWheel { device_id, delta, phase } => {
				self.input_engine().on_mouse_scroll(window_id, device_id, delta);
			},
			_ => ()
		}
	}
}
