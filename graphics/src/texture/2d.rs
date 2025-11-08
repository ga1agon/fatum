use crate::texture::{Filter, Format, WrapMode};

pub trait Texture2D {
	fn bind(&mut self, unit: usize);

	fn handle(&self) -> u64;
	fn filter(&self) -> Filter;
	fn wrap_mode(&self) -> WrapMode;
	fn format(&self) -> Format;
}
