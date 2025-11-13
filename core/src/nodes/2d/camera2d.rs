use fatum_scene::{Node, NodeComponent, NodeId, SharedSceneGraph};
use glam::{UVec2, Vec2};

use crate::components::{self, Transform2D};

#[derive(NodeComponent, Clone)]
pub struct Camera2D {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,
	size: UVec2
}

impl Camera2D {
	pub fn new(size: UVec2, active: bool) -> Node {
		let camera = fatum_graphics::Camera2D {
			position: Vec2::ZERO,
			up: fatum_graphics::Camera2D::UP,
			size
		};

		let mut node = Node::new();

		let c2d = Box::new(Self {
			owner: Default::default(),
			scene: Default::default(),
			size
		});

		let c = Box::new(components::Camera::new(camera.create(), active));
		let t2d = Box::new(Transform2D::default());

		node.add_component(c2d);
		node.add_component(c);
		node.add_component(t2d);

		node.connect_mut("$update", |args: &(*mut Node, std::time::Duration)| {
			let node = unsafe { &mut *args.0 };

			let t2d: Transform2D;

			{
				if let Some(c) = node.component::<Transform2D>() {
					t2d = c.clone();
				} else {
					return;
				}
			}

			let c2d: Self;

			{
				if let Some(c) = node.component::<Self>() {
					c2d = c.clone();
				} else {
					return;
				}
			}

			if let Some(camera) = node.component_mut::<components::Camera>() {
				camera.set_camera(fatum_graphics::Camera2D {
					position: t2d.translation(),
					up: t2d.rotation() * fatum_graphics::Camera2D::UP,
					size: c2d.size
				}.create());
			}
		});

		node
	}

	pub fn size(&self) -> UVec2 { self.size }
	pub fn set_size(&mut self, size: UVec2) { self.size = size }
}
