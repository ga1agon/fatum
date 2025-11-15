use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::task::Context;
use egui_glow::{EguiGlow, ShaderVersion};
use fatum_graphics::platform::{GraphicsContext, GraphicsPlatform, PlatformId};
use fatum_graphics::platform::opengl::OpenGlPlatform;
use fatum_scene::iterators::{SceneDfsIterator, ScenePostDfsIterator};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use crate::{GraphicsEngine, SceneEngine};
use crate::components::UiElement;

pub struct UiEngine<P: GraphicsPlatform> {
	graphics: Rc<RefCell<GraphicsEngine<P>>>,
	scene: Rc<RefCell<SceneEngine<P>>>,
	ui_glow: Option<EguiGlow>
}

impl<P> UiEngine<P> where P: GraphicsPlatform {
	pub fn new(event_loop: &ActiveEventLoop, graphics: Rc<RefCell<GraphicsEngine<P>>>, scene: Rc<RefCell<SceneEngine<P>>>) -> Self {
		let mut ui_glow: Option<EguiGlow> = None;

		{
			let mut graphics = graphics.borrow_mut();
			let graphics = graphics.get();

			match P::id() {
				PlatformId::OpenGL => {
					let graphics = graphics.as_any().downcast_ref::<OpenGlPlatform>().unwrap();
					let context = graphics.context();
					let gl = context.get();
					
					ui_glow = Some(EguiGlow::new(
						&event_loop,
						gl,
						None,
						None,
						true
					));
				},
				_ => todo!()
			}
		}

		log::info!("Created UI engine");

		Self {
			graphics,
			scene,
			ui_glow
		}
	}

	pub fn process(&mut self, window: WindowId, delta: std::time::Duration) -> bool {
		let scene_engine = self.scene.borrow();
		let graphics_engine = self.graphics.borrow();

		let queue_index = graphics_engine.queue_of_window(window);

		if queue_index.is_none() {
			log::warn!("Window {:?} does not belong to any queue", window);
			return false;
		}

		let queue_index = queue_index.unwrap();
		let scene = scene_engine.scene(queue_index);

		if scene.is_none() {
			log::debug!("Queue {:?} does not have any scene active", queue_index);
			return false;
		}

		let scene = scene.unwrap();
		let window = graphics_engine.window(window).unwrap().wimpl();

		let nodes: Vec<u32> = SceneDfsIterator::new(scene.clone(), Default::default())
			.collect();

		if let Ok(scene) = scene.try_read() {
			if let Some(ui) = self.ui_glow.as_mut() {
				ui.run(window, move |ctx| {
					for node in &nodes {
						let node = scene.node(*node)
							.expect("Iterator returned a non-existing node");

						if let Some(element) = node.component::<UiElement>() {
							element.draw(delta, ctx);
						}
					}
				});
			}
		} else {
			log::warn!("Could not get a read lock on scene {}; UI nodes will not be processed", queue_index);
		}

		if let Some(ui) = self.ui_glow.as_mut() {
			ui.paint(window);
		}

		true
	}

	pub fn on_window_event(&mut self, window: WindowId, event: &WindowEvent) {
		let graphics_engine = self.graphics.borrow();

		if let Some(window) = graphics_engine.window(window) {
			if let Some(ui_glow) = &mut self.ui_glow {
				_ = ui_glow.on_window_event(window.wimpl(), event);
			}
		}
	}
}
