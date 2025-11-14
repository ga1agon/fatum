use std::collections::HashMap;

use dear_imgui_glow::{GlTexture, TextureMap};
use dear_imgui_rs::{TextureData, TextureId, TextureStatus};

#[derive(Default)]
pub struct OglImGuiTextureMap {
	textures: HashMap<TextureId, GlTexture>,
	texture_data: HashMap<TextureId, TextureData>,
	next_id: usize
}

impl OglImGuiTextureMap {
	pub fn new() -> Self {
		Self {
			textures: HashMap::new(),
			texture_data: HashMap::new(),
			next_id: 0,
		}
	}
}

impl TextureMap for OglImGuiTextureMap {
	fn get(&self, texture_id: TextureId) -> Option<GlTexture> {
		self.textures.get(&texture_id).copied()
	}

	fn set(&mut self, texture_id: TextureId, gl_texture: GlTexture) {
		self.textures.insert(texture_id, gl_texture);
	}

	fn remove(&mut self, texture_id: TextureId) -> Option<GlTexture> {
		let gl_texture = self.textures.remove(&texture_id);
		self.texture_data.remove(&texture_id);
		gl_texture
	}

	fn clear(&mut self) {
		self.textures.clear();
		self.texture_data.clear();
	}

	fn register_texture(
		&mut self,
		gl_texture: GlTexture,
		width: i32,
		height: i32,
		format: dear_imgui_rs::TextureFormat,
	) -> TextureId {
		self.next_id += 1;
		let texture_id = TextureId::new(self.next_id as u64);

		let mut boxed = TextureData::new();

		boxed.create(format, width, height);

		boxed.set_tex_id(texture_id);
		boxed.set_status(TextureStatus::OK);

		let texture_data = *boxed;

		self.textures.insert(texture_id, gl_texture);
		self.texture_data.insert(texture_id, texture_data);

		texture_id
	}

	fn update_texture(
		&mut self,
		texture_id: TextureId,
		gl_texture: GlTexture,
		width: i32,
		height: i32,
	) {
		self.textures.insert(texture_id, gl_texture);

		if let Some(texture_data) = self.texture_data.get_mut(&texture_id) {
			texture_data.set_tex_id(texture_id);
			texture_data.set_status(TextureStatus::OK);
		}
	}

	fn get_texture_data(&self, texture_id: TextureId) -> Option<&TextureData> {
		self.texture_data.get(&texture_id)
	}

	fn get_texture_data_mut(&mut self, texture_id: TextureId) -> Option<&mut TextureData> {
		self.texture_data.get_mut(&texture_id)
	}
}
