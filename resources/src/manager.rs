use std::{cell::RefCell, collections::HashMap, fs::File, io::{BufReader, Read}, path::{Component, Path, PathBuf}, rc::Rc};

use crate::{Resource, ResourceMetadata, ResourcePlatform, Rf, error::{ErrorKind, ResourceError}, rf};

pub struct Resources<Pl> where Pl: ResourcePlatform {
	pub platform: Rc<Pl>,
	assets_directory: PathBuf,
	resources_by_id: HashMap<u64, Rf<Box<dyn Resource<Pl>>>>,
	resources_by_path: HashMap<PathBuf, Rf<Box<dyn Resource<Pl>>>>,
}

impl<Pl> Resources<Pl> where Pl: ResourcePlatform {
	pub fn new<P: AsRef<Path>>(platform: Rc<Pl>, assets_directory: P) -> Self {
		Self {
			platform,
			assets_directory: assets_directory.as_ref().to_path_buf(),
			resources_by_id: HashMap::new(),
			resources_by_path: HashMap::new()
		}
	}

	pub fn load_by_path<T, M, P>(&mut self, path: P, cache: bool) -> Result<Rf<Box<T>>, ResourceError>
		where
			T: Resource<Pl> + 'static,
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
			//return Ok(cached_resource.as_any().downcast_ref().unwrap());
			let borrowed = cached_resource.borrow();

			if let Some(concrete) = (&**borrowed).as_any().downcast_ref::<T>() {
				let resource_rf = unsafe {
					std::mem::transmute::<Rf<Box<dyn Resource<Pl>>>, Rf<Box<T>>>(cached_resource.clone())
				};

				return Ok(resource_rf);
			} else {
				return Err(ResourceError::new(
					ErrorKind::TypeMismatchError,
					"The cached resource's type does not match the requested resource's type"
				));
			}
		}

		let metadata = File::open(&metadata_path)
			.map_or(None, |f| Some(f));
		let asset = File::open(&asset_path)
			.map_err(|e| ResourceError::new(ErrorKind::IoError, format!("Failed to open asset: {}", e).as_str()))?;

		let resource = T::load(self, asset_path, metadata, &asset)?;
		let resource_rf = rf(Box::new(resource));

		if cache {
			let resource_dyn = unsafe {
				//let ptr = &raw const resource_rf as *const Rc<RefCell<Box<dyn Resource<Pl>>>>;
				let ptr = Rc::into_raw(resource_rf.clone()) as *const RefCell<Box<dyn Resource<Pl>>>;
				Rc::from_raw(ptr)
			};

			self.resources_by_id.insert(resource_rf.borrow().metadata().id(), resource_dyn.clone());
			self.resources_by_path.insert(metadata_path, resource_dyn.clone());
		}

		Ok(resource_rf)
	}

	pub fn unload<T>(&mut self, resource: &Rf<Box<T>>) where T: Resource<Pl> + 'static, dyn Resource<Pl>: PartialEq<T> {
		self.resources_by_id.retain(|_, v| **v.borrow() != **resource.borrow());
		self.resources_by_path.retain(|_, v| **v.borrow() != **resource.borrow());
	}
}
