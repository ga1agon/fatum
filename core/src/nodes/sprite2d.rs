use std::{cell::RefCell, rc::Rc};

use fatum_graphics::{Color, Material, Mesh, Model, Vertex, render::RenderObject, texture::Texture2D};
use fatum_macros::node_impl_new;
use fatum_scene::{Node, NodeBehaviour};
use glam::{Vec2, Vec3};
use crate::{nodes::Node2D, resources::ResTexture2D};

#[derive(NodeBehaviour)]
pub struct Sprite2D {
	pub base: Node2D,
	pub object: Rc<RenderObject>
}

impl Sprite2D {
	pub fn new(texture: Rc<RefCell<Box<ResTexture2D>>>) -> Self {
		let UNIT_QUAD: Model = Model {
			meshes: vec![
				Mesh {
					vertices: vec![
						Vertex::new(Vec3::new(0.0, 0.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(0.0, 0.0)),
						Vertex::new(Vec3::new(0.0, 1.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(0.0, 1.0)),
						Vertex::new(Vec3::new(1.0, 1.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(1.0, 1.0)),
						Vertex::new(Vec3::new(1.0, 0.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(1.0, 0.0)),
					],
					indices: vec![
						0u32, 1u32, 2u32,
						0u32, 2u32, 3u32
					],
					material: Material::default()
				}
			]
		};

		let base = Node2D::new();

		let material = Material::with_textures_pbr(
			Color::from_rgb_u8(255, 255, 255),
			0.0,
			1.0,
			1.5,
			Some(texture.borrow().get()),
			None,
			None,
			None,
			None
		);

		let mut model = UNIT_QUAD.clone();
		model.meshes[0].material = material;

		let translation = Vec3::new(0.0, 0.0, 0.0);
		let rotation = glam::Quat::from_euler(glam::EulerRot::YXZ, 0.0, 0.0, 0.0);
		let scale = Vec3::new(1.0, 1.0, 1.0);
		let matrix = glam::Mat4::from_scale_rotation_translation(scale, rotation, translation);

		let object = RenderObject::new(model, matrix);

		Self {
			base,
			object: object.into()
		}
	}
}

impl Into<Node> for Sprite2D {
	fn into(self) -> Node {
		self.base.into()
	}
}
