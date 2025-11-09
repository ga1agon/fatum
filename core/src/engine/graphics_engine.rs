use std::{collections::HashMap, rc::Rc, sync::Arc};

use fatum_graphics::{platform::GraphicsPlatform, render::{PipelineKind, RenderQueue, RenderTarget}};
use glam::UVec2;

use crate::{ApplicationInfo, CoreEngine, OutputKind};

pub struct GraphicsEngine<P: GraphicsPlatform> {
	app_info: ApplicationInfo,
	platform: P,

	outputs: HashMap<usize, (Box<dyn RenderQueue>, u8)>, // TODO remove the tuples
}

impl<P> GraphicsEngine<P> where P: GraphicsPlatform {
	pub fn new(app_info: ApplicationInfo) -> Self {
		let platform = P::new();
		log::info!("Created graphics engine ({})", std::any::type_name::<P>());

		Self {
			app_info,
			platform,
			outputs: HashMap::new()
		}
	}

	pub fn get(&mut self) -> &mut P { &mut self.platform }

	pub fn create_output(&mut self, index: usize, pipeline_kind: PipelineKind, output_kind: OutputKind) -> usize {
		log::info!("Creating output {} for {:?}", index, pipeline_kind);

		// this portion is so messy
		let mut output = self.outputs.get_mut(&index);
		let queue: &mut Box<dyn RenderQueue>;

		if let Some(output) = output {
			queue = &mut output.0;
		} else {
			self.outputs.insert(index, (self.platform.create_queue(), 0u8));
			output = self.outputs.get_mut(&index);
			queue = &mut output.unwrap().0;

			queue.set_pipeline(Some(self.platform.create_pipeline(pipeline_kind)));
		}

		let target: Box<dyn RenderTarget> = match output_kind {
			OutputKind::Window => {
				let mut window = self.platform.create_window(format!("{} ({})", self.app_info.name, index).as_str(), UVec2::new(1024, 768)).unwrap();
				window.show();
				window
			},
			_ => todo!()
		};

		let queue_target = queue.add_target(target);
		queue_target
	}

	pub fn process(&mut self) -> bool {
		for (_, (queue, _)) in &mut self.outputs {
			if !queue.is_active() {
				continue;
			}

			queue.process();
		}

		self.outputs.iter().all(|o| {
			o.1.0.is_active()
		})
	}
}
