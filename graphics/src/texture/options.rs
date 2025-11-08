#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
	Linear,
	Nearest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
	ClampToBorder,
	ClampToEdge,
	Repeat,
	RepeatMirror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
	RGBA8 = 4,
	RGB8 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options {
	pub filter: Filter,
	pub wrap_mode: WrapMode,
	pub format: Format,
}
