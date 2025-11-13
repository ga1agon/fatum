use std::{any::Any, cell::RefCell, fmt::Debug, fs::File, io::{BufReader, Write}, path::PathBuf, rc::Rc, sync::atomic::Ordering};

use fatum_graphics::{platform::GraphicsPlatform, texture::{self, Texture2D}};
use fatum_resources::{Resource, ResourceMetadata, ResourcePlatform, error::ResourceError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaTexture2D {
	pub id: u64,
	pub format: String,
	
	pub options: texture::Options
}

impl ResourceMetadata for MetaTexture2D {
	fn default() -> Self where Self: Sized {
		Self {
			id: fatum_resources::RESOURCE_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
			format: String::from("texture2d"),
			options: Default::default()
		}
	}

	fn id(&self) -> u64 { self.id }
	fn format(&self) -> &str { &self.format }
}

pub struct ResTexture2D {
	path: PathBuf,
	metadata: MetaTexture2D,

	value: Box<dyn Texture2D>
}

impl ResTexture2D {
	pub fn get(&self) -> &Box<dyn Texture2D> { &self.value }
}

impl<P: GraphicsPlatform + ResourcePlatform + Sized> Resource<P> for ResTexture2D {
	fn load(manager: &fatum_resources::Resources<P>, path: PathBuf, metadata: Option<File>, asset: File) -> Result<Self, fatum_resources::error::ResourceError>
		where Self: Sized
	{
		let file_reader = BufReader::new(asset);
		let image = image::ImageReader::new(file_reader)
			.with_guessed_format().unwrap()
			.decode()
			.map_err(|e| ResourceError::new(&path, fatum_resources::error::ErrorKind::LoadError, format!("Could not decode image: {}", e).as_str()))?;

		let metadata =
			if metadata.is_some() {
				ron::de::from_reader(metadata.unwrap())
					.map_err(|e| ResourceError::new(&path, fatum_resources::error::ErrorKind::MetadataError, format!("Failed to deserialize the resource metadata file: {}", e).as_str()))?
			} else {
				MetaTexture2D::default()
			};

		let value = manager.platform.create_texture_2d(image, metadata.options)
			.map_err(|e| ResourceError::new(&path, fatum_resources::error::ErrorKind::Other, &e.msg))?;

		Ok(Self {
			path,
			metadata,
			value
		})
	}

	fn save(&self, path: PathBuf, mut metadata: File, _: File) -> Result<(), ResourceError> {
		let metadata_value = ron::ser::to_string(&self.metadata)
			.map_err(|e| ResourceError::new(&path, fatum_resources::error::ErrorKind::MetadataError, format!("Failed to serialize resource metadata: {}", e).as_str()))?;

		metadata.write_all(metadata_value.as_bytes())
			.map_err(|e| ResourceError::new(&path, fatum_resources::error::ErrorKind::IoError, format!("Failed to write to metadata file: {}", e).as_str()))?;

		// we cannot & as such dont save the texture as a file

		Ok(())
	}

	fn reload(&mut self) {
		todo!()
	}

	fn path(&self) -> &std::path::PathBuf { &self.path }
	fn metadata(&self) -> &dyn fatum_resources::ResourceMetadata { &self.metadata }
	
	fn as_any(&self) -> &dyn Any { self }
}

// impl<Pl: ResourcePlatform> PartialEq<dyn Resource<Pl> + 'static> for ResTexture2D {
// 	fn eq(&self, other: &dyn Resource<Pl>) -> bool {
// 		self.path == *other.path()
// 	}
// }

#[cfg(debug_assertions)]
impl Drop for ResTexture2D {
	fn drop(&mut self) {
		log::debug!("{} was dropped", self.path.display());
	}
}
