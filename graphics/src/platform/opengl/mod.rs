mod window;
mod render_queue;
mod shader;
mod shader_program;
mod shader_data;
mod pipeline;
mod render_target;
mod texture_2d;

use std::{cell::RefCell, hash::Hash, rc::Rc};

use bytemuck::Pod;
use glam::{UVec2, Vec2};
use glow::HasContext;

pub use window::*;
pub use render_queue::*;
pub use shader::*;
pub use shader_program::*;
pub use shader_data::*;
pub use render_target::*;
pub use texture_2d::*;

use crate::{error::{ErrorKind, PlatformError}, platform::{GraphicsContext, GraphicsPlatform, opengl::pipeline::OpenGlPBRPipeline}, render::{PipelineKind, RenderPipeline, RenderTarget}, shader::*, texture, window::Window};
use glfw::{Context, PWindow, WindowHint, WindowMode};

// struct ContextWindow(Rc<PWindow>);

// impl PartialEq for ContextWindow {
// 	fn eq(&self, other: &Self) -> bool {
// 		self.0.window_id() == other.0.window_id()
// 	}
// }

// impl Eq for ContextWindow {}

// impl Hash for ContextWindow {
// 	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
// 		self.0.window_id().hash(state);
// 	}
// }

// impl Clone for ContextWindow {
// 	fn clone(&self) -> Self {
// 		Self(self.0.clone())
// 	}
// }

#[derive(Clone)]
pub struct OpenGlContext {
	gl: Rc<glow::Context>,
	glfw: Rc<RefCell<glfw::Glfw>>,

	shared_window: Option<Rc<OpenGlWindow>>,
}

impl GraphicsContext<glow::Context> for OpenGlContext {
	fn get(&self) -> Rc<glow::Context> { self.gl.clone() }
	fn glfw(&self) -> Rc<RefCell<glfw::Glfw>> { self.glfw.clone() }

	fn create_shader_data<D: Pod>(&self, program: &Box<dyn ShaderProgram>, name: &str, binding: u32, data: Option<Rc<Vec<D>>>) -> Result<Box<dyn ShaderData<D>>, PlatformError> {
		Ok(Box::new(OpenGlShaderData::new(&self.clone(), program, name, binding, data)?))
	}
}

#[derive(Clone)]
pub struct OpenGlPlatform {
	context: Rc<OpenGlContext>
}

impl GraphicsPlatform for OpenGlPlatform {
	fn new() -> Self {
		#[cfg(debug_assertions)]
		{
			// force GLFW to use X11 in debug as renderdoc doesn't support wayland
			glfw::init_hint(glfw::InitHint::Platform(glfw::Platform::X11));
		}

		let mut glfw = glfw::init(glfw::fail_on_errors)
			.unwrap();

		glfw.window_hint(WindowHint::Visible(false));
		glfw.window_hint(WindowHint::ContextVersion(3, 3));
		glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
		glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

		#[cfg(debug_assertions)]
		{
			glfw.window_hint(WindowHint::OpenGlDebugContext(true));
		}

		let (mut window, event_receiver) = glfw.create_window(1, 1, "", WindowMode::Windowed)
			.unwrap();

		window.make_current();
		
		let mut render_context = window.render_context();
		let mut gl = unsafe {
			glow::Context::from_loader_function(|s| render_context.get_proc_address(s).unwrap() as *const _)
		};

		// enable debug output
		unsafe {
			gl.enable(glow::DEBUG_OUTPUT);
			gl.enable(glow::DEBUG_OUTPUT_SYNCHRONOUS);
			
			gl.debug_message_callback(|source, msg_type, id, severity, message| {
				let severity = match severity {
					glow::DEBUG_SEVERITY_HIGH => "HIGH",
					glow::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
					glow::DEBUG_SEVERITY_LOW => "LOW",
					glow::DEBUG_SEVERITY_NOTIFICATION => "NOTIFY",
					_ => "?"
				};

				let msg_type = match msg_type {
					glow::DEBUG_TYPE_ERROR => "Error",
					glow::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated",
					glow::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined",
					glow::DEBUG_TYPE_PORTABILITY => "Portability",
					glow::DEBUG_TYPE_PERFORMANCE => "Performance",
					glow::DEBUG_TYPE_MARKER => "Marker",
					glow::DEBUG_TYPE_PUSH_GROUP => "Push group",
					glow::DEBUG_TYPE_POP_GROUP => "Pop group",
					glow::DEBUG_TYPE_OTHER => "Other",
					_ => "Unknown"
				};

				let source = match source {
					glow::DEBUG_SOURCE_API => "API",
					glow::DEBUG_SOURCE_WINDOW_SYSTEM => "Window system",
					glow::DEBUG_SOURCE_SHADER_COMPILER => "Shader compiler",
					glow::DEBUG_SOURCE_THIRD_PARTY => "Third-party",
					glow::DEBUG_SOURCE_APPLICATION => "Application",
					glow::DEBUG_SOURCE_OTHER => "Other",
					_ => "Unknown"
				};

				println!("OpenGL message {} [{}, type={}, source={}]: {}", id, severity, msg_type, source, message);
			});

			gl.debug_message_control(
				glow::DONT_CARE,
				glow::DONT_CARE,
				glow::DONT_CARE,
				&[],
				true
			);
		};
		
		let mut context = OpenGlContext {
			gl: Rc::new(gl),
			glfw: Rc::new(RefCell::new(glfw)),
			shared_window: None,
		};

		let base_window = Rc::new(OpenGlWindow::from_impl(Rc::new(context.clone()), window, event_receiver, ""));
		context.shared_window = Some(base_window);

		Self {
			context: Rc::new(context)
		}
	}

	//fn context(&self) -> Rc<OpenGlContext> { self.context.clone() }

	fn create_window(&mut self, title: &str, size: UVec2) -> Result<Box<dyn Window>, PlatformError> {
		let window = OpenGlWindow::new(self.context.clone(), title, size, &self.context.shared_window.as_ref().unwrap())?;
		Ok(Box::new(window))
	}
	
	fn create_queue(&self) -> Box<dyn crate::render::RenderQueue> {
		Box::new(OpenGlRenderQueue::new(self.context.clone()))
	}
	
	fn create_shader(&self, family: ShaderFamily, source: &str) -> Box<dyn Shader> {
		Box::new(OpenGlShader::new(&self.context.clone(), family, source))
	}
	
	fn create_shader_program(&self, shaders: Vec<Box<dyn Shader>>) -> Box<dyn ShaderProgram> {
		Box::new(OpenGlShaderProgram::new(&self.context.clone(), shaders))
	}

	fn create_texture_2d(&self, image: image::DynamicImage, options: texture::Options) -> Result<Box<dyn texture::Texture2D>, PlatformError> {
		Ok(Box::new(OglTexture2D::new(&self.context.clone(), image, options)?))
	}
	
	fn create_pipeline(&self, kind: PipelineKind) -> Box<dyn RenderPipeline> {
		match kind {
			PipelineKind::Default | PipelineKind::PBR => Box::new(OpenGlPBRPipeline::new(&self.context.clone(), self))
		}
	}
}

// TODO a way that would make resources not a required dependency of graphics?
impl fatum_resources::ResourcePlatform for OpenGlPlatform {}
