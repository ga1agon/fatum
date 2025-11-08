use std::fmt;

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
	pub fn new(kind: ErrorKind, msg: &str) -> Self {
		Self {
			kind,
			msg: msg.to_string()
		}
	}
}

impl<'a> fmt::Display for ResourceError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Resource error {:?}: {}", self.kind, self.msg)
	}
}
