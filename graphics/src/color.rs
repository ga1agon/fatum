use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Pod, Zeroable, PartialEq)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32
}

impl Color {
	pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self {
			r: r as f32 / 255.0,
			g: g as f32 / 255.0,
			b: b as f32 / 255.0,
			a: a as f32 / 255.0
		}
	}

	pub fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self {
		Self::from_rgba_u8(r, g, b, 255)
	}

	pub fn from_rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self {
			r,
			g,
			b,
			a
		}
	}

	pub fn from_rgb_f32(r: f32, g: f32, b: f32) -> Self {
		Self::from_rgba_f32(r, g, b, 1.0)
	}
}
