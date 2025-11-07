use std::{num::NonZeroU32, rc::Rc};

use glow::{HasContext, NativeProgram, NativeShader};

use crate::{error::{ErrorKind, PlatformError}, platform::{GraphicsContext, opengl::OpenGlContext}, shader::{Shader, ShaderProgram}};

pub struct OpenGlShaderProgram {
	gl: Rc<glow::Context>,

	handle: Option<NativeProgram>,
	shaders: Vec<Box<dyn Shader>>
}

impl OpenGlShaderProgram {
	pub fn new(context: &OpenGlContext, shaders: Vec<Box<dyn Shader>>) -> Self {
		Self {
			gl: context.get(),
			handle: None,
			shaders
		}
	}
}

impl ShaderProgram for OpenGlShaderProgram {
	fn bind(&self) {
		unsafe {
			self.gl.use_program(self.handle);
		}
	}

	fn build(&mut self) -> Result<u64, PlatformError> {
		assert_eq!(self.handle, None);
		assert_ne!(self.shaders.len(), 0);

		unsafe {
			let program = self.gl.create_program()
				.map_err(|e| PlatformError::new(ErrorKind::ProgramCreateError, format!("Failed to create the GL shader program: {}", e).as_str()))?;

			for shader in &mut self.shaders {
				if shader.handle() == 0 {
					_ = shader.compile()?
				}

				self.gl.attach_shader(
					program,
					NativeShader(
						NonZeroU32::new(shader.handle() as u32).unwrap()
					)
				);
			}

			self.gl.link_program(program);

			if !self.gl.get_program_link_status(program) {
				return Err(
					PlatformError::new(
						ErrorKind::ProgramLinkError,
						"Failed to link the GL shader program"
					)
				);
			}

			for shader in &mut self.shaders {
				self.gl.detach_shader(
					program,
					NativeShader(
						NonZeroU32::new(shader.handle() as u32).unwrap()
					)
				);
			}

			self.handle = Some(program);
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

impl Drop for OpenGlShaderProgram {
	fn drop(&mut self) {
		unsafe {
			if let Some(handle) = self.handle.take() {
				self.gl.delete_program(handle);
			}
		}
	}
}
