use std::path::PathBuf;

use fatum_resources::{ResourcePlatform, Resources};

pub struct ResourceEngine<P: ResourcePlatform> {
	resources: Resources<P>
}

impl<P> ResourceEngine<P> where P: ResourcePlatform + Clone {
	pub fn new(platform: &P, base_directory: &PathBuf) -> Self {
		let resources = Resources::new(platform.clone().into(), base_directory.join("assets"));
		log::info!("Created resource engine ({})", resources.assets_directory().display());

		Self {
			resources
		}
	}

	pub fn get(&mut self) -> &mut Resources<P> { &mut self.resources }
}
