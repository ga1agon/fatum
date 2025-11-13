use fatum_scene::{Node, NodeComponent, NodeId, SharedSceneGraph};
use glam::{Mat3, Mat4, Quat, UVec2, Vec3};

use crate::{components::{Transform, Transform3D}, helpers::mat4_decompose};

#[derive(NodeComponent, Clone)]
pub struct Camera {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,
	camera: fatum_graphics::Camera,
	active: bool
}

impl Camera {
	pub fn new(camera: fatum_graphics::Camera, active: bool) -> Self {
		Self {
			owner: Default::default(),
			scene: Default::default(),
			camera,
			active
		}
	}

	pub fn camera(&self) -> fatum_graphics::Camera { self.camera }
	pub fn set_camera(&mut self, camera: fatum_graphics::Camera) { self.camera = camera }

	pub fn is_active(&self) -> bool { self.active }
	pub fn set_active(&mut self, active: bool) { self.active = active }
}

impl Into<fatum_graphics::Camera> for Camera {
	fn into(self) -> fatum_graphics::Camera {
		self.camera
	}
}

impl Into<fatum_graphics::Camera> for &Camera {
	fn into(self) -> fatum_graphics::Camera {
		self.camera
	}
}
