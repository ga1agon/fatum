use std::{any::Any, fs::File, path::{Path, PathBuf}, sync::atomic::{AtomicU64, AtomicUsize}};

use crate::{ResourceMetadata, ResourcePlatform, Resources, error::ResourceError};

pub trait Resource<Pl> where Pl: ResourcePlatform + Sized {
	fn load(manager: &Resources<Pl>, path: PathBuf, metadata: Option<File>, asset: &File) -> Result<Self, ResourceError>
		where Self: Sized;
	
	fn save(&self);
	fn reload(&mut self);

	fn path(&self) -> &PathBuf;
	fn metadata(&self) -> &dyn ResourceMetadata;

	fn as_any(&self) -> &dyn Any;
}
