mod data;
mod program;

pub use data::*;
pub use program::*;

use crate::error::PlatformError;

pub trait Shader {
	//fn new(family: ShaderFamily, source: &str) -> Self;
	
	fn compile(&mut self) -> Result<u64, PlatformError>;
	
	fn handle(&self) -> u64;
}

pub enum ShaderFamily {

	Vertex,
	Fragment,
	Geometry,
	Compute,
	TessellationControl,
	TessellationEval
}
