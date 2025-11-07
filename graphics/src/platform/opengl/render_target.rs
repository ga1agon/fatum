use std::collections::HashMap;

pub struct RenderTargetResources {
	pub vaos: HashMap<u64, Vec<glow::NativeVertexArray>>,
}

impl RenderTargetResources {
	pub fn new() -> Self {
		Self {
			vaos: HashMap::new()
		}
	}
}
