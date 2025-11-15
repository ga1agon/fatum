use std::{num::{NonZero, NonZeroU32}, rc::Rc, sync::Arc, u8};

use bytemuck::Pod;
use glow::HasContext;

use crate::{error::{ErrorKind, PlatformError}, platform::{GraphicsContext, opengl::OpenGlContext}, shader::{ShaderData, ShaderProgram}};

pub struct OpenGlShaderData<D> {
	gl: Arc<glow::Context>,
	handle: Option<glow::NativeBuffer>,

	name: String,
	binding: u32,
	data: Option<Rc<Vec<D>>>
}

impl<D> OpenGlShaderData<D> {
	pub fn new(context: &OpenGlContext, program: &Box<dyn ShaderProgram>, name: &str, binding: u32, data: Option<Rc<Vec<D>>>) -> Result<Self, PlatformError> {
		unsafe {
			let gl = context.get();

			let program_handle = glow::NativeProgram(NonZeroU32::new(program.handle() as u32).unwrap());

			let block_index = gl.get_uniform_block_index(program_handle, name)
				.ok_or(PlatformError::new(ErrorKind::NoBufferBlockError, &format!("No such buffer block with name {}", name)))?;

			let handle = gl.create_buffer()
				.map_err(|e| PlatformError::new(ErrorKind::BufferCreateError, format!("Failed to create GL buffer: {}", e).as_str()))?;
			
			gl.bind_buffer(glow::UNIFORM_BUFFER, Some(handle));
			gl.uniform_block_binding(
				program_handle,
				block_index,
				binding
			);
			
			Ok(Self {
				handle: Some(handle),
				gl,
				name: name.to_string(),
				binding,
				data
			})
		}
	}
}

impl<D> ShaderData<D> for OpenGlShaderData<D>
	where D: bytemuck::Pod
{
	fn push(&self) {
		assert_ne!(self.handle, None);

		unsafe {
			self.gl.bind_buffer(glow::UNIFORM_BUFFER, self.handle);

			let raw_data = match &self.data {
				Some(data) => bytemuck::cast_slice(&*data),
				None => &[]
			};

			self.gl.buffer_data_u8_slice(glow::UNIFORM_BUFFER, raw_data, glow::STATIC_DRAW);
			self.gl.bind_buffer_base(glow::UNIFORM_BUFFER, self.binding, self.handle);
		}
	}

	fn handle(&self) -> u64 {
		if self.handle.is_none() {
			return 0
		}

		self.handle.unwrap().0.get() as u64
	}
	
	fn name(&self) -> &str { &self.name }
	fn binding(&self) -> u32 { self.binding }

	fn set_data(&mut self, data: Rc<Vec<D>>) { self.data = Some(data.clone()) }
}
