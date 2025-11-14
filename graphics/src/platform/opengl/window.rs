use std::{cell::RefCell, hash::Hash, num::NonZeroU32, rc::Rc};

use glam::UVec2;
use glow::HasContext;
use glutin::{api::egl::config, config::{Api, ConfigTemplateBuilder, GlConfig}, context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext}, display::GetGlDisplay, prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext}, surface::{GlSurface, Surface, SwapInterval, WindowSurface}};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::{dpi::LogicalSize, platform::x11::EventLoopBuilderExtX11, raw_window_handle::HasRawWindowHandle, window::WindowAttributes};

use crate::{error::{ErrorKind, PlatformError}, platform::{GraphicsContext, opengl::{OpenGlContext, RenderTargetResources}}, render::RenderTarget, window::Window};

pub struct OpenGlWindow {
	context: Rc<OpenGlContext>,
	wimpl: winit::window::Window,
	weloop: winit::event_loop::EventLoop<()>,

	pub gl: glow::Context,
	pub gl_surface: Surface<WindowSurface>,
	pub gl_context: PossiblyCurrentContext,

	title: String,

	resources: RenderTargetResources,
}

impl OpenGlWindow {
	// TODO how to actually share contexts? idfk
	pub fn new(context: Rc<OpenGlContext>, title: &str, size: UVec2, parent: Option<&OpenGlWindow>) -> Result<Self, PlatformError> {
		let mut window_attributes = WindowAttributes::default()
			.with_title(title)
			.with_inner_size(LogicalSize::new(size.x, size.y))
			.with_visible(false);
		
		if let Some(parent) = parent {
			window_attributes = unsafe {
				window_attributes.with_parent_window(Some(parent.wimpl().raw_window_handle().unwrap()))
			};
		}

		let event_loop = winit::event_loop::EventLoop::builder()
			.build()
			.map_err(|e| PlatformError::new(ErrorKind::WindowCreateError, format!("Failed to create event loop: {}", e).as_str()))?;

		let template = ConfigTemplateBuilder::default()
			.with_api(Api::OPENGL)
			.with_transparency(true);

		let (window, gl_config) = DisplayBuilder::default()
			.with_window_attributes(Some(window_attributes))
			.build(&event_loop, template, |configs| {
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
		
		#[cfg(debug_assertions)]
		{
			context_attributes = context_attributes.with_debug(true)
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

		

		Ok(Self {
			context,
			wimpl: window,
			weloop: event_loop,
			gl,
			gl_surface,
			gl_context,
			title: title.to_string(),
			resources: RenderTargetResources::new()
		})
	}

	// pub fn from_impl(
	// 	context: Rc<OpenGlContext>,
	// 	window_impl: glfw::PWindow,
	// 	event_receiver: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
	// 	title: &str,
	// ) -> Self {
	// 	Self {
	// 		context,
	// 		window_impl,
	// 		event_receiver,
	// 		title: title.to_string(),
	// 		resources: RenderTargetResources::new()
	// 	}
	// }

	pub fn resources(&self) -> &RenderTargetResources { &self.resources }
	pub fn resources_mut(&mut self) -> &mut RenderTargetResources { &mut self.resources }
}

impl Window for OpenGlWindow {
	fn wimpl(&self) -> &winit::window::Window {
		&self.wimpl
	}

	fn weloop(&self) -> &winit::event_loop::EventLoop<()> {
		&self.weloop
	}

	fn title(&self) -> &str { &self.title }
	fn set_title(&mut self, title: &str) { self.title = title.to_string() }

	fn show(&mut self) {
		self.wimpl.set_visible(true);
	}

	fn hide(&mut self) {
		self.wimpl.set_visible(false);
	}

	fn close(self) { }

	fn should_close(&self) -> bool {
		false
	}
}

impl RenderTarget for OpenGlWindow {
	fn begin(&mut self) {
		self.gl_context.make_current(&self.gl_surface).unwrap();

		let gl = self.context.get();
		
		unsafe {
			let size = RenderTarget::size(self);
			gl.viewport(0, 0, size.x as i32, size.y as i32);
		}
	}

	fn end(&mut self) {
		self.gl_surface.swap_buffers(&self.gl_context).unwrap();
		self.weloop.run(event_handler)
		//self.context.glfw().borrow_mut().poll_events();
	}

	fn size(&self) -> UVec2 {
		let size = self.wimpl.inner_size();

		UVec2 {
			x: size.width,
			y: size.height
		}
	}

	// fn set_size(&mut self, size: UVec2) {
	// 	self.window_impl.set_size(size.x as i32, size.y as i32);
	// }

	fn is_active(&self) -> bool {
		!self.should_close()
	}

	fn set_active(&mut self, active: bool) {
		//self.window_impl.set_should_close(!active);
	}

	fn as_any(&self) -> &dyn std::any::Any { self }
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

impl PartialEq for OpenGlWindow {
	fn eq(&self, other: &Self) -> bool {
		self.wimpl.id() == other.wimpl.id()
	}
}

impl Eq for OpenGlWindow {}

impl Hash for OpenGlWindow {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.wimpl.id().hash(state);
	}
}
