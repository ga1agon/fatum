use std::{cell::RefCell, rc::Rc, hash::Hash};

use glam::UVec2;
use glfw::{Context, PWindow, WindowHint, WindowMode};
use glow::HasContext;

use crate::{error::{ErrorKind, PlatformError}, platform::{GraphicsContext, opengl::{OpenGlContext, RenderTargetResources}}, render::RenderTarget, window::Window};

pub struct OpenGlWindow {
	context: Rc<OpenGlContext>,
	pub(crate) window_impl: glfw::PWindow,
	pub(crate) event_receiver: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

	title: String,

	resources: RenderTargetResources,
}

impl OpenGlWindow {
	pub fn new(context: Rc<OpenGlContext>, title: &str, size: UVec2, shared_window: &OpenGlWindow) -> Result<Self, PlatformError> {
		// let mut m_glfw = glfw.borrow_mut();

		// m_glfw.window_hint(WindowHint::Visible(false));
		// m_glfw.window_hint(WindowHint::ContextVersion(3, 3));
		// m_glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
		// m_glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

		let (mut window_impl, event_receiver) = shared_window.window_impl.create_shared(size.x, size.y, title, WindowMode::Windowed)
			.ok_or(PlatformError::new(ErrorKind::WindowCreateError, "Failed to create GLFW window"))?;

		window_impl.make_current();

		Ok(Self {
			context,
			window_impl,
			event_receiver,
			title: title.to_string(),
			resources: RenderTargetResources::new()
		})
	}

	pub fn from_impl(
		context: Rc<OpenGlContext>,
		window_impl: glfw::PWindow,
		event_receiver: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
		title: &str,
	) -> Self {
		Self {
			context,
			window_impl,
			event_receiver,
			title: title.to_string(),
			resources: RenderTargetResources::new()
		}
	}

	pub fn resources(&self) -> &RenderTargetResources { &self.resources }
	pub fn resources_mut(&mut self) -> &mut RenderTargetResources { &mut self.resources }
}

impl Window for OpenGlWindow {
	fn title(&self) -> &str { &self.title }
	fn set_title(&mut self, title: &str) { self.title = title.to_string() }

	fn show(&mut self) {
		self.window_impl.show();
	}

	fn hide(&mut self) {
		self.window_impl.hide();
	}

	fn close(self) { }

	fn should_close(&self) -> bool {
		self.window_impl.should_close()
	}
}

impl RenderTarget for OpenGlWindow {
	fn begin(&mut self) {
		self.window_impl.make_current();

		let gl = self.context.get();
		
		unsafe {
			let size = RenderTarget::size(self);
			gl.viewport(0, 0, size.x as i32, size.y as i32);
		}
	}

	fn end(&mut self) {
		self.window_impl.swap_buffers();
		self.context.glfw().borrow_mut().poll_events();
	}

	fn size(&self) -> UVec2 {
		let size = self.window_impl.get_size();

		UVec2 {
			x: size.0 as u32,
			y: size.1 as u32
		}
	}

	// fn set_size(&mut self, size: UVec2) {
	// 	self.window_impl.set_size(size.x as i32, size.y as i32);
	// }

	fn is_active(&self) -> bool {
		!self.should_close()
	}

	fn set_active(&mut self, active: bool) {
		self.window_impl.set_should_close(!active);
	}

	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

impl PartialEq for OpenGlWindow {
	fn eq(&self, other: &Self) -> bool {
		self.window_impl.window_id() == other.window_impl.window_id()
	}
}

impl Eq for OpenGlWindow {}

impl Hash for OpenGlWindow {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.window_impl.window_id().hash(state);
	}
}
