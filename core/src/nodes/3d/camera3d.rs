use fatum_scene::{Node, NodeComponent, NodeId, SharedSceneGraph};
use glam::{Mat3, Mat4, Quat, UVec2, Vec3};

use crate::{components::{self, Transform, Transform3D}, helpers::mat4_decompose};

#[derive(NodeComponent, Clone)]
pub struct Camera3D {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,
	size: UVec2,
	fov: f32
}

impl Camera3D {
	pub fn new_perspective(size: UVec2, fov: f32, active: bool) -> Node {
		let camera = fatum_graphics::Camera3D {
			position: Vec3::ZERO,
			target: Vec3::ZERO,
			up: fatum_graphics::Camera3D::UP,
			size,
			fov
		};

		let mut node = Node::new();

		let c3d = Box::new(Self {
			owner: Default::default(),
			scene: Default::default(),
			size,
			fov
		});

		let c = Box::new(components::Camera::new(camera.create_perspective(), active));
		let t3d = Box::new(Transform3D::default());

		node.add_component(c3d);
		node.add_component(c);
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

			let c3d: Self;

			{
				if let Some(c) = node.component::<Self>() {
					c3d = c.clone();
				} else {
					return;
				}
			}

			if let Some(camera) = node.component_mut::<components::Camera>() {
				let forward = t3d.rotation() * fatum_graphics::Camera3D::FRONT;
				let up = t3d.rotation() * fatum_graphics::Camera3D::UP;

				let target = t3d.translation() + forward;

				camera.set_camera(fatum_graphics::Camera3D {
					position: t3d.translation(),
					target,
					up,
					size: c3d.size,
					fov: c3d.fov
				}.create_perspective());
			}
		});

		node
	}

	pub fn size(&self) -> UVec2 { self.size }
	pub fn set_size(&mut self, size: UVec2) { self.size = size }

	//pub fn target(&self) -> Vec3 { self.camera.target }
	pub fn set_target(&mut self, target: Vec3) {
		if self.scene.is_none() {
			return;
		}

		let scene = self.scene.clone().unwrap();
		let mut scene = scene.write().unwrap();
		let owner = scene.node_mut(self.owner).unwrap();

		if let Some(t3d) = owner.component_mut::<Transform3D>() {
			let forward = target - t3d.translation();
			let right = (t3d.rotation() * fatum_graphics::Camera3D::UP).cross(forward).normalize();
			let up = forward.cross(right);

			let rotation_matrix = Mat3::from_cols(right, up, forward);
			let rotation = Quat::from_mat3(&rotation_matrix);

			t3d.set_rotation(rotation);
		}
	}

	pub fn fov(&self) -> f32 { self.fov }
	pub fn set_fov(&mut self, fov: f32) { self.fov = fov }

	// pub fn is_active(&self) -> bool { self.active }
	// pub fn set_active(&mut self, active: bool) { self.active = active }
}
