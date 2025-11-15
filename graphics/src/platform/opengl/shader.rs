use std::{rc::Rc, sync::Arc};

use glow::{HasContext, NativeShader};

use crate::{error::*, platform::{GraphicsContext, opengl::{OpenGlContext, OpenGlPlatform}}, shader::{Shader, ShaderFamily}};

pub struct OpenGlShader {
	gl: Arc<glow::Context>,

	handle: Option<NativeShader>,
	family: ShaderFamily,
	source: String
}

impl OpenGlShader {
	pub fn new(context: &OpenGlContext, family: ShaderFamily, source: &str) -> Self {
		Self {
			gl: context.get(),
			handle: None,
			family,
			source: source.to_string()
		}
	}
}

impl Shader for OpenGlShader {
	fn compile(&mut self) -> Result<u64, PlatformError> {
		assert_eq!(self.handle, None);
		assert_eq!(self.source.is_empty(), false);

		unsafe {
			let shader = self.gl.create_shader(match self.family {
				ShaderFamily::Vertex => glow::VERTEX_SHADER,
				ShaderFamily::Fragment => glow::FRAGMENT_SHADER,
				ShaderFamily::Geometry => glow::GEOMETRY_SHADER,
				ShaderFamily::Compute => glow::COMPUTE_SHADER,
				ShaderFamily::TessellationControl => glow::TESS_CONTROL_SHADER,
				ShaderFamily::TessellationEval => glow::TESS_EVALUATION_SHADER,
			}).map_err(|e| PlatformError::new(ErrorKind::ShaderCreateError, format!("Failed to create the GL shader: {}", e).as_str()))?;

			self.handle = Some(shader);
			
			self.gl.shader_source(shader, &self.source);
			self.gl.compile_shader(shader);

			if !self.gl.get_shader_compile_status(shader) {
				return Err(
					PlatformError::new(
						ErrorKind::ShaderCompileError,
						self.gl.get_shader_info_log(shader).as_str()
					)
				);
			}
		}

		Ok(self.handle.unwrap().0.get() as u64)
	}

	fn handle(&self) -> u64 {
		if self.handle.is_none() {
			return 0
		}

		self.handle.unwrap().0.get() as u64
	}
}

impl Drop for OpenGlShader {
	fn drop(&mut self) {
		unsafe {
			if let Some(handle) = self.handle.take() {
				self.gl.delete_shader(handle);
			}
		}
	}
}
