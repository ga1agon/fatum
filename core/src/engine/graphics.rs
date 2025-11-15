use std::{collections::HashMap, rc::Rc, sync::Arc};

use fatum_graphics::{platform::GraphicsPlatform, render::{PipelineKind, RenderQueue, RenderTarget}};
use glam::UVec2;
use winit::event_loop::EventLoop;

use crate::{ApplicationInfo, CoreEngine, OutputKind};

pub struct GraphicsEngine<P: GraphicsPlatform> {
	app_info: ApplicationInfo,
	platform: P,

	outputs: HashMap<usize, Box<dyn RenderQueue>>,
}

impl<P> GraphicsEngine<P> where P: GraphicsPlatform {
	pub fn new(event_loop: &EventLoop<()>, app_info: ApplicationInfo) -> Self {
		let platform = P::new(event_loop).unwrap();
		log::info!("Created graphics engine ({})", std::any::type_name::<P>());

		Self {
			app_info,
			platform,
			outputs: HashMap::new()
		}
	}

	pub fn get(&mut self) -> &mut P { &mut self.platform }

	pub fn create_output(&mut self, event_loop: &EventLoop<()>, index: usize, pipeline_kind: PipelineKind, output_kind: OutputKind) -> usize {
		log::info!("Creating output {} for {:?}", index, pipeline_kind);

		// this portion is so messy
		let mut output = self.outputs.get_mut(&index);
		let queue: &mut Box<dyn RenderQueue>;

		if let Some(output) = output {
			queue = output;
		} else {
			self.outputs.insert(index, self.platform.create_queue());
			output = self.outputs.get_mut(&index);
			queue = output.unwrap();

			queue.set_pipeline(Some(self.platform.create_pipeline(pipeline_kind)));
		}

		let target: Box<dyn RenderTarget> = match output_kind {
			OutputKind::Window => {
				let mut window = self.platform.create_window(event_loop, format!("{} ({})", self.app_info.name, index).as_str(), UVec2::new(1024, 768)).unwrap();
				window.show();
				window
			},
			_ => todo!()
		};

		let queue_target = queue.add_target(target);
		queue_target
	}

	pub fn output(&mut self, index: usize) -> Option<&mut Box<dyn RenderQueue>> {
		self.outputs.get_mut(&index)
	}

	pub fn outputs(&mut self) -> &mut HashMap<usize, Box<dyn RenderQueue>> {
		&mut self.outputs
	}

	pub fn is_active(&self) -> bool {
		self.outputs.iter().all(|o| {
			o.1.is_active()
		})
	}

	pub fn process(&mut self, _: std::time::Duration) {
		for (_, queue) in &mut self.outputs {
			if !queue.is_active() {
				continue;
			}

			_ = queue.process();
		}
	}
}
