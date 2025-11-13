mod texture2d;
pub use texture2d::*;

mod text;
pub use text::*;

mod action_map;
pub use action_map::*;

#[macro_export]
macro_rules! serialize_metadata {
	($metadata: expr, $path: expr) => {
		ron::ser::to_string(&$metadata)
			.map_err(|e| fatum_resources::error::ResourceError::new(&$path, fatum_resources::error::ErrorKind::MetadataError, format!("Failed to serialize resource metadata: {}", e).as_str()))
	};
}

#[macro_export]
macro_rules! deserialize_metadata {
	($metadata: expr, $path: expr, $default: expr) => {
		if $metadata.is_some() {
			ron::de::from_reader($metadata.unwrap())
				.map_err(|e| fatum_resources::error::ResourceError::new(&$path, fatum_resources::error::ErrorKind::MetadataError, format!("Failed to deserialize resource metadata: {}", e).as_str()))?
		} else {
			$default
		}
	};
}

#[macro_export]
macro_rules! write_resource_file {
	($file: expr, $path: expr, $val: expr) => {
		$file.write_all($val)
			.map_err(|e| fatum_resources::error::ResourceError::new(&$path, fatum_resources::error::ErrorKind::IoError, format!("Failed to write to resource file: {}", e).as_str()))
	};
}
