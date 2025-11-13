use fatum_graphics::platform::GraphicsPlatform;
use fatum_resources::ResourcePlatform;

use crate::CoreEngine;

#[derive(Debug, Clone)]
pub struct ApplicationInfo {
	pub name: String
}

impl Default for ApplicationInfo {
	fn default() -> Self {
		Self {
			name: String::from("Fatum")
		}
	}
}

pub trait Application<P: GraphicsPlatform + ResourcePlatform> {
	fn info() -> ApplicationInfo;

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>) where Self: Sized;
	fn process(&mut self, engine: &mut CoreEngine<P, Self>, delta: std::time::Duration) where Self: Sized {}
}
