use std::{collections::HashMap, io::Write, path::PathBuf, sync::atomic::Ordering};

use fatum_resources::{Resource, ResourceMetadata, ResourcePlatform, error::{ErrorKind, ResourceError}};
use serde::{Deserialize, Serialize};

use crate::{deserialize_metadata, input::{ActionMap, InputAction, InputCombo}, serialize_metadata, write_resource_file};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaActionMap {
	pub id: u64,
	pub format: String,
}

impl ResourceMetadata for MetaActionMap {
	fn default() -> Self where Self: Sized {
		Self {
			id: fatum_resources::RESOURCE_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
			format: "action_map".to_string()
		}
	}

	fn id(&self) -> u64 { self.id }
	fn format(&self) -> &str { &self.format }
}

#[derive(Clone)]
pub struct ResActionMap {
	path: PathBuf,
	metadata: MetaActionMap,
	value: ActionMap
}

impl ResActionMap {
	pub fn new(value: ActionMap) -> Self {
		Self {
			path: Default::default(),
			metadata: MetaActionMap::default(),
			value
		}
	}
	
	pub fn get(&self) -> &ActionMap { &self.value }
	pub fn get_mut(&mut self) -> &mut ActionMap { &mut self.value }
}

impl<P: ResourcePlatform + Sized> Resource<P> for ResActionMap {
	fn load(manager: &fatum_resources::Resources<P>, path: PathBuf, metadata: Option<std::fs::File>, asset: std::fs::File) -> Result<Self, fatum_resources::error::ResourceError>
		where Self: Sized
	{
		let asset: HashMap<Vec<InputCombo>, String> = ron::de::from_reader(asset)
			.map_err(|e| ResourceError::new(&path, ErrorKind::IoError, format!("Failed to deserialize action map: {}", e).as_str()))?;

		let mut value = HashMap::new();

		for (combos, action_name) in asset {
			let action = InputAction::new(&action_name);
			value.insert(combos, action);
		}

		let metadata = deserialize_metadata!(metadata, path, MetaActionMap::default());
		
		Ok(Self {
			path,
			metadata,
			value
		})
	}

	fn save(&self, path: PathBuf, mut metadata: std::fs::File, mut asset: std::fs::File) -> Result<(), ResourceError> {
		let mut value = HashMap::new();

		for (combos, action) in &self.value {
			value.insert(combos, action.borrow().name().to_string());
		}

		let value = ron::ser::to_string(&value)
			.map_err(|e| ResourceError::new(&path, fatum_resources::error::ErrorKind::SerializationError, format!("Failed to serialize asset value: {}", e).as_str()))?;

		let metadata_value = serialize_metadata!(self.metadata, path)?;

		write_resource_file!(metadata, path, metadata_value.as_bytes())?;
		write_resource_file!(asset, path, value.as_bytes())?;

		Ok(())
	}

	fn reload(&mut self) {
		todo!()
	}

	fn path(&self) -> &PathBuf { &self.path }
	fn metadata(&self) -> &dyn ResourceMetadata { &self.metadata }

	fn as_any(&self) -> &dyn std::any::Any { self }
}
