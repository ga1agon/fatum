mod window;
mod render_queue;
mod shader;
mod shader_program;
mod shader_data;
mod pipeline;
mod render_target;
mod texture_2d;

use std::{cell::RefCell, hash::Hash, num::NonZeroU32, rc::Rc};

use bytemuck::Pod;
use glam::{UVec2, Vec2};
use glow::HasContext;

use glutin::{config::{Api, ConfigTemplateBuilder, GlConfig}, context::{AsRawContext, ContextApi, ContextAttributesBuilder, PossiblyCurrentContext}, display::GetGlDisplay, prelude::{GlDisplay, NotCurrentGlContext}, surface::{GlSurface, Surface, SwapInterval, WindowSurface}};
use glutin_winit::{DisplayBuilder, GlWindow};
pub use window::*;
pub use render_queue::*;
pub use shader::*;
pub use shader_program::*;
pub use shader_data::*;
pub use render_target::*;
pub use texture_2d::*;
use winit::{dpi::LogicalSize, event_loop::{EventLoop, EventLoopBuilder}, platform::{x11::EventLoopBuilderExtX11}, raw_window_handle::HasRawWindowHandle, window::{Window, WindowAttributes}};

use crate::{RenderWindow, error::{ErrorKind, PlatformError}, platform::{GraphicsContext, GraphicsPlatform, opengl::pipeline::OpenGlPBRPipeline}, render::{PipelineKind, RenderPipeline, RenderTarget}, shader::*, texture};

#[derive(Clone)]
pub struct OpenGlContext {
	gl: Rc<glow::Context>,
	glutin: Rc<RefCell<PossiblyCurrentContext>>,
}

impl GraphicsContext<glow::Context> for OpenGlContext {
	fn get(&self) -> Rc<glow::Context> { self.gl.clone() }

	fn create_shader_data<D: Pod>(&self, program: &Box<dyn ShaderProgram>, name: &str, binding: u32, data: Option<Rc<Vec<D>>>) -> Result<Box<dyn ShaderData<D>>, PlatformError> {
		Ok(Box::new(OpenGlShaderData::new(&self.clone(), program, name, binding, data)?))
	}
}

#[derive(Clone)]
pub struct OpenGlPlatform {
	context: Rc<OpenGlContext>,
}

impl OpenGlPlatform {
	fn create_window(event_loop: &EventLoop<()>, title: &str, size: UVec2, shared_context: Option<&PossiblyCurrentContext>)
		-> Result<(Window, Surface<WindowSurface>, PossiblyCurrentContext, glow::Context), PlatformError>
	{
		let window_attributes = WindowAttributes::default()
			.with_title(title)
			.with_inner_size(LogicalSize::new(size.x, size.y))
			.with_visible(false);

		let template = ConfigTemplateBuilder::default()
			.with_api(Api::OPENGL)
			.with_transparency(true);

		let (window, gl_config) = DisplayBuilder::default()
			.with_window_attributes(Some(window_attributes))
			.build(event_loop, template, |configs| {
				configs.reduce(|accum, config| {
					if config.hardware_accelerated() && !accum.hardware_accelerated() {
						if config.num_samples() > accum.num_samples() {
							config
						} else {
							accum
						}
					} else {
						accum
					}
				}).unwrap()
			}).map_err(|e| PlatformError::new(ErrorKind::WindowCreateError, format!("Failed to create window: {}", e).as_str()))?;

		let window = window.unwrap();

		let gl_display = gl_config.display();
		let mut context_attributes = ContextAttributesBuilder::default()
			.with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
				major: 3,
				minor: 3
			})));
		
		{
			if let Some(shared_context) = shared_context {
				context_attributes = context_attributes.with_sharing(shared_context);
			}
			
			#[cfg(debug_assertions)]
			{
				context_attributes = context_attributes.with_debug(true)
			}
		}

		let context_attributes = context_attributes.build(Some(window.raw_window_handle().unwrap()));
		
		let gl_context = unsafe {
			gl_display.create_context(&gl_config, &context_attributes)
				.map_err(|e| PlatformError::new(ErrorKind::WindowCreateError, format!("Failed to create GL context: {}", e).as_str()))?
		};

		let surface_attributes = window.build_surface_attributes(Default::default())
			.map_err(|e| PlatformError::new(ErrorKind::WindowCreateError, format!("Failed to build surface attributes: {}", e).as_str()))?;

		let gl_surface = unsafe {
			gl_display.create_window_surface(&gl_config, &surface_attributes)
				.map_err(|e| PlatformError::new(ErrorKind::WindowCreateError, format!("Failed to create window surface: {}", e).as_str()))?
		};

		let gl_context = gl_context.make_current(&gl_surface)
			.map_err(|e| PlatformError::new(ErrorKind::WindowCreateError, format!("Couldn't make context current: {}", e).as_str()))?;
		
		gl_surface
			.set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
			.unwrap();

		let gl = unsafe {
			glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s))
		};

		Ok((window, gl_surface, gl_context, gl))
	}
}

impl GraphicsPlatform for OpenGlPlatform {
	fn id() -> super::PlatformId {
		super::PlatformId::OpenGL
	}

	fn new(event_loop: &EventLoop<()>) -> Result<Self, PlatformError> {
		let (root_window, _, context, mut gl) = Self::create_window(&event_loop, "", UVec2::new(512, 512), None)
			.map_err(|e| PlatformError::new(ErrorKind::PlatformInitError, format!("Failed to create the root window: {}", e).as_str()))?;

		// enable debug output
		#[cfg(debug_assertions)]
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
		
		let context = OpenGlContext {
			gl: Rc::new(gl),
			glutin: Rc::new(RefCell::new(context))
		};

		Ok(Self {
			context: Rc::new(context)
		})
	}

	fn create_window(&mut self, event_loop: &EventLoop<()>, title: &str, size: UVec2) -> Result<Box<dyn RenderWindow>, PlatformError> {
		let (window, surface, _, _) = Self::create_window(
			event_loop,
			title,
			size,
			Some(&*self.context.glutin.borrow())
		)?;

		let window = OpenGlWindow::new(self.context.clone(), window, surface);
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
