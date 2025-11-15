use std::{cell::RefCell, hash::Hash, num::NonZeroU32, rc::Rc};

use glam::UVec2;
use glow::HasContext;
use glutin::{api::egl::config, config::{Api, ConfigTemplateBuilder, GlConfig}, context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext}, display::GetGlDisplay, prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext}, surface::{GlSurface, Surface, SwapInterval, WindowSurface}};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::{dpi::LogicalSize, platform::x11::EventLoopBuilderExtX11, raw_window_handle::HasRawWindowHandle, window::WindowAttributes};

use crate::{RenderWindow, error::{ErrorKind, PlatformError}, platform::{GraphicsContext, opengl::{OpenGlContext, RenderTargetResources}}, render::RenderTarget};

pub struct OpenGlWindow {
	context: Rc<OpenGlContext>,
	wimpl: winit::window::Window,
	active: bool,

	pub gl: Rc<glow::Context>,
	pub gl_context: Rc<RefCell<PossiblyCurrentContext>>,
	pub gl_surface: Surface<WindowSurface>,

	resources: RenderTargetResources,
}

impl OpenGlWindow {
	pub fn new(
		context: Rc<OpenGlContext>,
		wimpl: winit::window::Window,
		gl_surface: Surface<WindowSurface>
	) -> Self {
		let context = context.clone();
		
		Self {
			context: context.clone(),
			active: false,
			wimpl,
			gl: context.gl.clone(),
			gl_context: context.glutin.clone(),
			gl_surface,
			resources: RenderTargetResources::new()
		}
	}

	pub fn resources(&self) -> &RenderTargetResources { &self.resources }
	pub fn resources_mut(&mut self) -> &mut RenderTargetResources { &mut self.resources }
}

impl RenderWindow for OpenGlWindow {
	fn wimpl(&self) -> &winit::window::Window {
		&self.wimpl
	}

	fn wimpl_mut(&mut self) -> &mut winit::window::Window {
		&mut self.wimpl
	}
}

impl RenderTarget for OpenGlWindow {
	fn begin(&mut self) {
		self.gl_context.borrow_mut().make_current(&self.gl_surface).unwrap();

		let gl = self.context.get();
		
		unsafe {
			let size = RenderTarget::size(self);
			gl.viewport(0, 0, size.x as i32, size.y as i32);
		}
	}

	fn end(&mut self) {
		self.gl_surface.swap_buffers(&*self.gl_context.borrow()).unwrap();
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
		self.active
	}

	fn set_active(&mut self, active: bool) {
		self.active = active
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
