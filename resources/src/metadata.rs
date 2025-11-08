use serde::{Deserialize, Serialize};

pub const METADATA_FILE_EXTENSION: &'static str = ".asset";

pub trait ResourceMetadata {
	fn default() -> Self where Self: Sized;

	fn id(&self) -> u64;
	fn format(&self) -> &str;
}
