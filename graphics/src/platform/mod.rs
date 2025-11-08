pub mod opengl;

use std::{cell::RefCell, rc::Rc};

use crate::{error::PlatformError, render::{PipelineKind, RenderPipeline, RenderQueue}, shader::{Shader, ShaderData, ShaderFamily, ShaderProgram}, texture::{self, Texture2D}, window::Window};
use bytemuck::Pod;
use glam::UVec2;

pub trait GraphicsContext<T> {
	fn get(&self) -> Rc<T>;
	fn glfw(&self) -> Rc<RefCell<glfw::Glfw>>;

	fn create_shader_data<D: Pod>(&self, program: &Box<dyn ShaderProgram>, name: &str, binding: u32, data: Option<Rc<Vec<D>>>)
		-> Result<Box<dyn ShaderData<D>>, PlatformError>;
}

pub trait GraphicsPlatform
// <T1, T2>
// 	where T1: GraphicsContext<T2>
{
	//fn context(&self) -> Rc<T1>;

	fn create_window(&mut self, title: &str, size: UVec2) -> Result<Box<dyn Window>, PlatformError>;
	fn create_queue(&self) -> Box<dyn RenderQueue>;

	fn create_shader(&self, family: ShaderFamily, source: &str) -> Box<dyn Shader>;
	fn create_shader_program(&self, shaders: Vec<Box<dyn Shader>>) -> Box<dyn ShaderProgram>;
	
	//fn create_array_shader_data<D: Pod>(&self, program: &Box<dyn ShaderProgram>, name: &str, binding: u32, data: Option<Rc<Vec<D>>>)
	//	-> Result<Box<dyn ShaderData<Vec<D>>>, PlatformError>;

	fn create_texture_2d(&self, image: image::DynamicImage, options: texture::Options) -> Result<Box<dyn Texture2D>, PlatformError>;

	fn create_pipeline(&self, kind: PipelineKind) -> Box<dyn RenderPipeline>;
}
