use fatum_scene::{Node, NodeComponent, NodeId, SharedSceneGraph};
use glam::{UVec2, Vec2};

use crate::components::Transform2D;

#[derive(NodeComponent)]
pub struct Camera2D {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,
	camera: fatum_graphics::Camera2D,
	active: bool
}

impl Camera2D {
	pub fn new(size: UVec2, active: bool) -> Self {
		let camera = fatum_graphics::Camera2D {
			position: Vec2::ZERO,
			size
		};

		Self {
			owner: Default::default(),
			scene: Default::default(),
			camera,
			active
		}
	}

	pub fn new_node(size: UVec2, active: bool) -> Node {
		let mut node = Node::new();

		let camera2d = Box::new(Self::new(size, active));
		let transform2d = Box::new(Transform2D::default());

		node.add_component(camera2d);
		node.add_component(transform2d);
		node
	}

	pub fn size(&self) -> UVec2 { self.camera.size }
	pub fn set_size(&mut self, size: UVec2) { self.camera.size = size }

	pub fn is_active(&self) -> bool { self.active }
	pub fn set_active(&mut self, active: bool) { self.active = active }
}

impl Into<fatum_graphics::Camera> for Camera2D {
	fn into(self) -> fatum_graphics::Camera {
		self.camera.create()
	}
}

impl Into<fatum_graphics::Camera> for &Camera2D {
	fn into(self) -> fatum_graphics::Camera {
		self.camera.create()
	}
}
