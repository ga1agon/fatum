use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
	Linear,
	Nearest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WrapMode {
	ClampToBorder,
	ClampToEdge,
	Repeat,
	RepeatMirror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Format {
	RGBA8 = 4,
	RGB8 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Options {
	pub filter: Filter,
	pub wrap_mode: WrapMode,
	pub format: Format,
}

impl Default for Options {
	fn default() -> Self {
		Self {
			filter: Filter::Linear,
			wrap_mode: WrapMode::Repeat,
			format: Format::RGBA8
		}
	}
}
