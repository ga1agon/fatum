use std::{io::Read, path::PathBuf, sync::atomic::Ordering};

use fatum_graphics::platform::GraphicsPlatform;
use fatum_resources::{Resource, ResourceMetadata, ResourcePlatform, error::ResourceError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaText {
	pub id: u64,
	pub format: String
}

impl ResourceMetadata for MetaText {
	fn default() -> Self where Self: Sized {
		Self {
			id: fatum_resources::RESOURCE_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
			format: String::from("text")
		}
	}

	fn id(&self) -> u64 { self.id }
	fn format(&self) -> &str { &self.format }
}

pub struct ResText {
	path: PathBuf,
	metadata: MetaText,

	value: String
}

impl ResText {
	pub fn get(&self) -> &str { &self.value }
}

impl<P: GraphicsPlatform + ResourcePlatform + Sized> Resource<P> for ResText {
	fn load(manager: &fatum_resources::Resources<P>, path: PathBuf, mut metadata: Option<std::fs::File>, mut asset: std::fs::File) -> Result<Self, fatum_resources::error::ResourceError>
		where Self: Sized
	{
		let mut value = String::new();

		asset.read_to_string(&mut value)
			.map_err(|e| ResourceError::new(&path, fatum_resources::error::ErrorKind::IoError, format!("Could not read text file: {}", e).as_str()))?;

		let metadata =
			if metadata.is_some() {
				ron::de::from_reader(metadata.unwrap())
					.map_err(|e| ResourceError::new(&path, fatum_resources::error::ErrorKind::MetadataError, format!("Failed to deserialize the resource metadata file: {}", e).as_str()))?
			} else {
				MetaText::default()
			};
		
		Ok(Self {
			path,
			metadata,
			value
		})
	}

	fn save(&self) {
		todo!()
	}

	fn reload(&mut self) {
		todo!()
	}

	fn path(&self) -> &PathBuf { &self.path }
	fn metadata(&self) -> &dyn ResourceMetadata { &self.metadata }
	fn as_any(&self) -> &dyn std::any::Any { self }
}
