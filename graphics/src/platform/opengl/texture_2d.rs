use std::{io::BufReader, path::PathBuf, rc::Rc};

use glam::UVec2;
use glow::{HasContext, NativeTexture, PixelUnpackData};
use image::{GenericImageView, RgbaImage, metadata::Orientation};

use crate::{error::{ErrorKind, PlatformError}, platform::{GraphicsContext, GraphicsPlatform, opengl::{OpenGlContext, OpenGlPlatform}}, texture::{self, Filter, Format, Texture2D, WrapMode}};

pub struct OglTexture2D {
	gl: Rc<glow::Context>,

	handle: NativeTexture,
	options: texture::Options,
}

impl OglTexture2D {
	pub fn new(context: &OpenGlContext, mut image: image::DynamicImage, options: texture::Options) -> Result<Self, PlatformError> {
		let gl = context.get();

		let handle = unsafe {
			gl.create_texture()
				.map_err(|e| PlatformError::new(ErrorKind::TextureCreateError, format!("Could not create GL texture: {}", e).as_str()))?
		};

		let mut inst = Self {
			gl: gl.clone(),
			handle,
			options,
		};

		inst.bind(0);

		let image_size = UVec2::new(image.dimensions().0, image.dimensions().1);

		let gl_filter = match options.filter {
			Filter::Linear => glow::LINEAR,
			Filter::Nearest => glow::NEAREST
		} as i32;

		let gl_wrap_mode = match options.wrap_mode {
			WrapMode::ClampToBorder => glow::CLAMP_TO_BORDER,
			WrapMode::ClampToEdge => glow::CLAMP_TO_EDGE,
			WrapMode::Repeat => glow::REPEAT,
			WrapMode::RepeatMirror => glow::MIRRORED_REPEAT
		} as i32;

		unsafe {
			gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, gl_filter);
			gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, gl_filter);
			gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, gl_wrap_mode);
			gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, gl_wrap_mode);
			gl.tex_parameter_f32_slice(glow::TEXTURE_2D, glow::TEXTURE_BORDER_COLOR, &[0.0, 0.0, 0.0, 0.0]);

			// TODO is this the most optimal way to do this?
			image.apply_orientation(Orientation::FlipVertical);
			let u8_image = image.to_rgba8();
			let pixel_data = u8_image.as_raw();

			gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA8 as i32,
				image_size.x as i32,
				image_size.y as i32,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				PixelUnpackData::Slice(Some(&pixel_data[..pixel_data.len()]))
			);
		};

		Ok(inst)
	}
}

impl Texture2D for OglTexture2D {
	fn bind(&mut self, unit: usize) {
		unsafe {
			self.gl.active_texture(glow::TEXTURE0 + unit as u32);
			self.gl.bind_texture(glow::TEXTURE_2D, Some(self.handle));
		}
	}

	fn handle(&self) -> u64 { self.handle.0.get() as u64 }
	fn options(&self) -> texture::Options { self.options }
}
