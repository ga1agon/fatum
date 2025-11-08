use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorKind {
	PlatformInitError,
	WindowCreateError,
	ShaderCreateError,
	ShaderCompileError,
	ProgramCreateError,
	ProgramLinkError,
	BufferCreateError,
	NoBufferBlockError,
	TextureCreateError,
}

#[derive(Debug, Clone)]
pub struct PlatformError {
	kind: ErrorKind,
	msg: String
}

impl PlatformError {
	pub fn new(kind: ErrorKind, msg: &str) -> Self {
		Self {
			kind,
			msg: msg.to_string()
		}
	}
}

impl<'a> fmt::Display for PlatformError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Platform error {:?}: {}", self.kind, self.msg)
	}
}
