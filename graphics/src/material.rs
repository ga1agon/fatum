use bytemuck::{Pod, Zeroable};

use crate::{Color, texture::Texture2D};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable, PartialEq)]
pub struct Material {
	pub base_color: Color,
	pub metalness: f32,
	pub roughness: f32,
	pub ior: f32,

	// texture maps
	pub map_0: u32,
	pub map_1: u32,
	pub map_2: u32,
	pub map_3: u32,
	pub map_4: u32
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
			..Default::default()
		}
	}

	pub fn with_color(color: Color) -> Self {
		Self {
			base_color: color,
			metalness: 0.0,
			roughness: 0.5,
			ior: 1.5,
			..Default::default()
		}
	}

	pub fn with_textures_pbr(
		base_color: Color,
		metalness: f32,
		roughness: f32,
		ior: f32,
		base_map: Option<&Box<dyn Texture2D>>,
		metalness_map: Option<&Box<dyn Texture2D>>,
		roughness_map: Option<&Box<dyn Texture2D>>,
		normal_map: Option<&Box<dyn Texture2D>>,
		displacement_map: Option<&Box<dyn Texture2D>>
	) -> Self {
		Self {
			base_color,
			metalness,
			roughness,
			ior,
			map_0: base_map.map_or(0, |t| t.handle() as u32),
			map_1: metalness_map.map_or(0, |t| t.handle() as u32),
			map_2: roughness_map.map_or(0, |t| t.handle() as u32),
			map_3: normal_map.map_or(0, |t| t.handle() as u32),
			map_4: displacement_map.map_or(0, |t| t.handle() as u32)
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
			map_0: 0,
			map_1: 0,
			map_2: 0,
			map_3: 0,
			map_4: 0
		}
	}
}
