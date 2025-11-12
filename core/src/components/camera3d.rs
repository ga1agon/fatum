use fatum_scene::{Node, NodeComponent, NodeId, SharedSceneGraph};
use glam::{Mat3, Mat4, Quat, UVec2, Vec3};

use crate::{components::{Transform, Transform3D}, helpers::mat4_decompose};

#[derive(NodeComponent, Clone)]
pub struct Camera3D {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,
	camera: fatum_graphics::Camera3D,
	active: bool
}

impl Camera3D {
	pub fn new_perspective(fov: f32, size: UVec2, active: bool) -> Self {
		let camera = fatum_graphics::Camera3D {
			position: Vec3::ZERO,
			target: Vec3::ZERO,
			up: fatum_graphics::Camera3D::UP,
			size,
			fov
		};

		Self {
			owner: Default::default(),
			scene: Default::default(),
			camera,
			active
		}
	}

	pub fn new_perspective_node(fov: f32, size: UVec2, active: bool) -> Node {
		let mut node = Node::new();

		let c3d = Box::new(Self::new_perspective(fov, size, active));
		let t3d = Box::new(Transform3D::default());

		node.add_component(c3d);
		node.add_component(t3d);

		node.connect_mut("$update", |args: &(*mut Node, std::time::Duration)| {
			let node = unsafe { &mut *args.0 };

			let t3d: Transform3D;

			{
				if let Some(c) = node.component::<Transform3D>() {
					t3d = c.clone();
				} else {
					return;
				}
			}

			if let Some(this) = node.component_mut::<Self>() {
				let (translation, rotation, _) = mat4_decompose(t3d.global_matrix());

				this.camera.position = translation;
				
				let forward = rotation * fatum_graphics::Camera3D::FRONT;
				let up = rotation * fatum_graphics::Camera3D::UP;

				this.camera.target = translation + forward;
				this.camera.up = up;
			}
		});
		node
	}

	pub fn size(&self) -> UVec2 { self.camera.size }
	pub fn set_size(&mut self, size: UVec2) { self.camera.size = size }

	pub fn target(&self) -> Vec3 { self.camera.target }
	pub fn set_target(&mut self, target: Vec3) {
		if self.scene.is_none() {
			self.camera.target = target
		}

		let scene = self.scene.clone().unwrap();
		let mut scene = scene.write().unwrap();
		let owner = scene.node_mut(self.owner).unwrap();

		if let Some(t3d) = owner.component_mut::<Transform3D>() {
			let forward = target - t3d.translation();
			let right = self.camera.up.cross(forward).normalize();
			let up = forward.cross(right);

			let rotation_matrix = Mat3::from_cols(right, up, forward);
			let rotation = Quat::from_mat3(&rotation_matrix);

			t3d.set_rotation(rotation);
		} else {
			self.camera.target = target
		}
	}

	pub fn fov(&self) -> f32 { self.camera.fov }
	pub fn set_fov(&mut self, fov: f32) { self.camera.fov = fov }

	pub fn is_active(&self) -> bool { self.active }
	pub fn set_active(&mut self, active: bool) { self.active = active }
}

impl Into<fatum_graphics::Camera> for Camera3D {
	fn into(self) -> fatum_graphics::Camera {
		self.camera.create_perspective()
	}
}

impl Into<fatum_graphics::Camera> for &Camera3D {
	fn into(self) -> fatum_graphics::Camera {
		self.camera.create_perspective()
	}
}
