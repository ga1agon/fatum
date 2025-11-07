use bytemuck::{Pod, Zeroable};

use crate::Color;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable, PartialEq)]
pub struct Material {
	pub base_color: Color,
	pub metalness: f32,
	pub roughness: f32,
	pub ior: f32,
	#[doc(hidden)] _padding0: f32
}

impl Material {
	pub fn new(
		base_color: Color,
		metalness: f32,
		roughness: f32,
		ior: f32
	) -> Self {
		Self {
			base_color,
			metalness,
			roughness,
			ior,
			_padding0: Default::default()
		}
	}

	pub fn with_color(color: Color) -> Self {
		Self {
			base_color: color,
			metalness: 0.0,
			roughness: 0.5,
			ior: 1.5,
			_padding0: Default::default()
		}
	}
}

impl Default for Material {
	fn default() -> Self {
		Self {
			base_color: Default::default(),
			metalness: 0.0,
			roughness: 0.5,
			ior: 1.5,
			_padding0: Default::default()
		}
	}
}
