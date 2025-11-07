use std::{cell::RefCell, rc::Rc, hash::Hash};

use glam::UVec2;
use glfw::{Context, PWindow, WindowHint, WindowMode};

use crate::{error::{ErrorKind, PlatformError}, platform::{GraphicsContext, opengl::{OpenGlContext, RenderTargetResources}}, render::RenderTarget, window::Window};

pub struct OpenGlWindow {
	context: Rc<OpenGlContext>,
	pub(crate) window_impl: glfw::PWindow,
	pub(crate) event_receiver: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

	title: String,
	size: UVec2,

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
			size,
			resources: RenderTargetResources::new()
		})
	}

	pub fn from_impl(
		context: Rc<OpenGlContext>,
		window_impl: glfw::PWindow,
		event_receiver: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
		title: &str,
		size: UVec2
	) -> Self {
		Self {
			context,
			window_impl,
			event_receiver,
			title: title.to_string(),
			size,
			resources: RenderTargetResources::new()
		}
	}

	pub fn resources(&self) -> &RenderTargetResources { &self.resources }
	pub fn resources_mut(&mut self) -> &mut RenderTargetResources { &mut self.resources }
}

impl Window for OpenGlWindow {
	fn title(&self) -> &str { &self.title }
	fn set_title(&mut self, title: &str) { self.title = title.to_string() }

	fn size(&self) -> UVec2 { self.size }
	fn set_size(&mut self, size: UVec2) { self.size = size }

	fn show(&mut self) {
		self.window_impl.show();
	}

	fn hide(&mut self) {
		self.window_impl.hide();
	}

	fn close(self) { }
}

impl RenderTarget for OpenGlWindow {
	fn begin(&mut self) {
		self.window_impl.make_current();

	}

	fn end(&mut self) {
		self.window_impl.swap_buffers();
		self.context.glfw().borrow_mut().poll_events();
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
