use std::{cell::RefCell, collections::HashMap, fs::File, io::{BufReader, Read}, path::{Component, Path, PathBuf}, rc::Rc};

use crate::{Resource, ResourceMetadata, ResourcePlatform, Rf, rf};

struct Resources {
	assets_directory: PathBuf,
	resources_by_id: HashMap<u64, Rf<Box<dyn Resource>>>,
	resources_by_path: HashMap<PathBuf, Rf<Box<dyn Resource>>>,
}

impl Resources {
	fn new<P: AsRef<Path>>(assets_directory: P) -> Self {
		Self {
			assets_directory: assets_directory.as_ref().to_path_buf(),
			resources_by_id: HashMap::new(),
			resources_by_path: HashMap::new()
		}
	}

	fn load_by_path<T, M, P>(&mut self, platform: &dyn ResourcePlatform, path: P, cache: bool) -> Result<Rf<Box<dyn Resource>>, std::io::Error>
		where
			T: Resource + 'static,
			M: ResourceMetadata + serde::de::DeserializeOwned,
			P: AsRef<Path>
	{
		let mut asset_path = self.assets_directory.clone();

		let components = path.as_ref().components().skip_while(|c| {
			matches!(c, Component::Prefix(_) | Component::RootDir)
		});

		for c in components {
			asset_path.push(c);
		}

		let metadata_path = asset_path.join(crate::METADATA_FILE_EXTENSION);

		if let Some(cached_resource) = self.resources_by_path.get(&metadata_path) {
			return Ok(cached_resource.clone());
		}

		let metadata: M =
			if let Ok(f) = File::open(&metadata_path) {
				ron::de::from_reader(f)
					.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to deserialize the resource metadata file"))?
			} else {
				M::default()
			};

		let asset = File::open(asset_path)?;
		let resource = T::load(metadata, &asset);
		let resource = rf(Box::new(resource) as Box<dyn Resource>);

		if cache {
			self.resources_by_id.insert(resource.borrow().metadata().id(), resource.clone());
			self.resources_by_path.insert(metadata_path, resource.clone());
		}

		Ok(resource.clone())
	}
}
