use crate::{platform::GraphicsPlatform, texture::{Filter, Format, Options, WrapMode}};

pub trait Texture2D {
	fn bind(&mut self, unit: usize);

	fn handle(&self) -> u32;
	fn options(&self) -> Options;
}
