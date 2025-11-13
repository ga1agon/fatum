use std::{io::{Read, Write}, path::PathBuf, sync::atomic::Ordering};

use fatum_graphics::platform::GraphicsPlatform;
use fatum_resources::{Resource, ResourceMetadata, ResourcePlatform, error::ResourceError};
use serde::{Deserialize, Serialize};

use crate::{deserialize_metadata, serialize_metadata, write_resource_file};

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
	pub fn new(value: &str) -> Self {
		Self {
			path: Default::default(),
			metadata: MetaText::default(),
			value: value.to_string()
		}
	}
	
	pub fn get(&self) -> &str { &self.value }
}

impl<P: GraphicsPlatform + ResourcePlatform + Sized> Resource<P> for ResText {
	fn load(manager: &fatum_resources::Resources<P>, path: PathBuf, mut metadata: Option<std::fs::File>, mut asset: std::fs::File) -> Result<Self, fatum_resources::error::ResourceError>
		where Self: Sized
	{
		let mut value = String::new();

		asset.read_to_string(&mut value)
			.map_err(|e| ResourceError::new(&path, fatum_resources::error::ErrorKind::IoError, format!("Could not read text file: {}", e).as_str()))?;

		let metadata = deserialize_metadata!(metadata, path, MetaText::default());
		
		Ok(Self {
			path,
			metadata,
			value
		})
	}

	fn save(&self, path: PathBuf, mut metadata: std::fs::File, mut asset: std::fs::File) -> Result<(), ResourceError> {
		let metadata_value = serialize_metadata!(self.metadata, path)?;

		write_resource_file!(metadata, path, metadata_value.as_bytes())?;
		write_resource_file!(asset, path, self.value.as_bytes())?;

		Ok(())
	}

	fn reload(&mut self) {
		todo!()
	}

	fn path(&self) -> &PathBuf { &self.path }
	fn metadata(&self) -> &dyn ResourceMetadata { &self.metadata }
	fn as_any(&self) -> &dyn std::any::Any { self }
}
