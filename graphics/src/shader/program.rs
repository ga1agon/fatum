use crate::{error::PlatformError, shader::Shader};

pub trait ShaderProgram {
	fn bind(&self);
	fn build(&mut self) -> Result<u64, PlatformError>;
	
	fn handle(&self) -> u64;
}
