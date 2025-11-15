use std::{collections::HashMap, io::pipe, rc::Rc, sync::Arc};

use fatum_graphics::{RenderWindow, platform::{GraphicsPlatform, PlatformId, opengl::OpenGlWindow}, render::{PipelineKind, RenderQueue, RenderTarget}};
use glam::UVec2;
use winit::{event_loop::EventLoop, window::WindowId};

use crate::{ApplicationInfo, CoreEngine, OutputKind};

pub struct GraphicsEngine<P: GraphicsPlatform> {
	app_info: ApplicationInfo,
	platform: P,

	queues: HashMap<usize, Box<dyn RenderQueue>>,
	windows: HashMap<WindowId, (usize, usize)>,
}

impl<P> GraphicsEngine<P> where P: GraphicsPlatform {
	pub fn new(event_loop: &EventLoop<()>, app_info: ApplicationInfo) -> Self {
		let platform = P::new(event_loop).unwrap();
		log::info!("Created graphics engine ({})", std::any::type_name::<P>());

		Self {
			app_info,
			platform,
			queues: HashMap::new(),
			windows: HashMap::new()
		}
	}

	pub fn get(&mut self) -> &mut P { &mut self.platform }

	pub fn create_queue(&mut self, index: usize, pipeline_kind: PipelineKind) {
		log::info!("Creating output {} ({:?})", index, pipeline_kind);
		assert!(!self.queues.contains_key(&index));

		let mut queue = self.platform.create_queue();
		let pipeline = self.platform.create_pipeline(pipeline_kind);

		queue.set_pipeline(Some(pipeline));

		self.queues.insert(index, queue);
	}

	pub fn create_output(&mut self, queue_index: usize, event_loop: &EventLoop<()>, kind: OutputKind) -> usize {
		log::info!("Creating {:?} output for queue {}", kind, queue_index);
		assert!(self.queues.contains_key(&queue_index));

		let queue = self.queues.get_mut(&queue_index).unwrap();

		match kind {
			OutputKind::Window => {
				let mut window = self.platform.create_window(event_loop, format!("{} ({})", self.app_info.name, queue_index).as_str(), UVec2::new(1024, 768)).unwrap();
				window.show();
				window.begin();

				let window_id = window.wimpl().id();

				let target_id = queue.add_target(window);
				self.windows.insert(window_id, (queue_index, target_id));

				return target_id;
			},
			_ => todo!()
		};
	}

	pub fn queue(&mut self, index: usize) -> Option<&mut Box<dyn RenderQueue>> {
		self.queues.get_mut(&index)
	}

	pub fn queues(&mut self) -> &mut HashMap<usize, Box<dyn RenderQueue>> {
		&mut self.queues
	}

	pub fn queue_of_window(&self, window: WindowId) -> Option<usize> {
		if let Some((queue_index, _)) = self.windows.get(&window) {
			return Some(*queue_index);
		}

		None
	}

	pub fn window(&self, id: WindowId) -> Option<&dyn RenderWindow> {
		if 
			let Some((queue_index, window_index)) = &self.windows.get(&id)
			&& let Some(queue) = self.queues.get(queue_index)
			&& let Some(target) = queue.get_target(*window_index)
		{
			match P::id() {
				PlatformId::OpenGL => {
					let window = target.as_any().downcast_ref::<OpenGlWindow>().unwrap();
					return Some(window as &dyn RenderWindow);
				},
				_ => todo!()
			}
		}

		None
	}

	pub fn window_mut(&mut self, id: WindowId) -> Option<&mut dyn RenderWindow> {
		if 
			let Some((queue_index, window_index)) = &self.windows.get(&id)
			&& let Some(queue) = self.queues.get_mut(queue_index)
			&& let Some(target) = queue.get_target_mut(*window_index)
		{
			match P::id() {
				PlatformId::OpenGL => {
					let window = target.as_any_mut().downcast_mut::<OpenGlWindow>().unwrap();
					return Some(window as &mut dyn RenderWindow);
				},
				_ => todo!()
			}
		}

		None
	}

	// pub fn window_mut(&mut self, id: WindowId) -> Option<&mut Box<dyn RenderWindow>> {
	// 	if 
	// 		let Some((queue_index, window_index)) = &self.windows.get(&id)
	// 		&& let Some(queue) = self.queues.get_mut(queue_index)
	// 		&& let Some(target) = queue.get_target_mut(*window_index)
	// 	{
	// 		return Some(target.as_any_mut().downcast_mut::<Box<dyn RenderWindow>>().unwrap());
	// 	}

	// 	None
	// }

	pub fn is_active(&self) -> bool {
		self.queues.iter().all(|o| {
			o.1.is_active()
		})
	}

	pub fn begin(&mut self, window: WindowId) -> bool {
		let (queue_index, target_id) = self.windows[&window];
		let queue = self.queues.get_mut(&queue_index).unwrap();

		if queue.is_active() {
			queue.begin_single(target_id);
			return true;
		}

		false
	}

	pub fn process(&mut self, window: WindowId) {
		let (queue_index, target_id) = self.windows[&window];
		let queue = self.queues.get_mut(&queue_index).unwrap();

		if queue.is_active() {
			queue.process_single(target_id);
		}
	}

	pub fn end(&mut self, window: WindowId) {
		let (queue_index, target_id) = self.windows[&window];
		let queue = self.queues.get_mut(&queue_index).unwrap();

		if queue.is_active() {
			queue.end_single(target_id);
		}
	}
}
