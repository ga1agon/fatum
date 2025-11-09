use std::{fmt, path::Path};

#[derive(Debug, Clone)]
pub enum ErrorKind {
	MetadataError,
	TypeMismatchError,
	IoError,
	LoadError,
	SaveError,
	Other
}

#[derive(Debug, Clone)]
pub struct ResourceError {
	kind: ErrorKind,
	msg: String
}

impl ResourceError {
	pub fn new<P: AsRef<Path>>(path: P, kind: ErrorKind, msg: &str) -> Self {
		Self {
			kind,
			msg: format!("{} -> {}", path.as_ref().to_str().unwrap_or("Unknown"), msg)
		}
	}
}

impl<'a> fmt::Display for ResourceError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Resource error {:?}: {}", self.kind, self.msg)
	}
}
