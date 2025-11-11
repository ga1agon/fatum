use std::{cell::{LazyCell, OnceCell, RefCell}, rc::Rc, sync::{Arc, Mutex}};

use fatum_graphics::{Color, Material, Mesh, Model, Vertex, render::RenderObject, texture::Texture2D};
use fatum_macros::node_impl_new;
use fatum_resources::ResourceRef;
use fatum_scene::{Node, NodeBehaviour, NodeComponent, NodeId, SceneGraph, SharedSceneGraph};
use glam::{Vec2, Vec3};
use lazy_static::lazy_static;
use static_init::dynamic;
use crate::{behaviours::{ObjectRenderable, Renderable}, components::{self, Transform2D}, nodes::Node2D, resources::ResTexture2D};

#[dynamic]
static UNIT_QUAD: Model = Model {
	meshes: vec![
		Mesh {
			vertices: vec![
				Vertex::new(Vec3::new(-0.5, -0.5, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(0.0, 0.0)),
				Vertex::new(Vec3::new(-0.5,  0.5, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(0.0, 1.0)),
				Vertex::new(Vec3::new( 0.5,  0.5, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(1.0, 1.0)),
				Vertex::new(Vec3::new( 0.5, -0.5, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(1.0, 0.0)),
			],
			indices: vec![
				0u32, 1u32, 2u32,
				0u32, 2u32, 3u32
			],
			material: Material::default()
		}
	]
};

#[derive(NodeComponent)]
pub struct Sprite2D {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,
	texture: ResourceRef<ResTexture2D>,
	model: Rc<Box<fatum_graphics::Model>>
}

impl Sprite2D {
	pub fn new(texture: ResourceRef<ResTexture2D>) -> Self {
		let mut model = Box::new(UNIT_QUAD.clone());
		model.meshes[0].material.map_0 = texture.borrow().get().handle();

		let model = Rc::new(model);

		Self {
			owner: 0,
			scene: None,
			texture,
			model
		}
	}

	pub fn new_node(texture: ResourceRef<ResTexture2D>) -> Node {
		let mut node = Node::new();

		let sprite2d = Box::new(Self::new(texture));
		let model = Box::new(components::Model::new(sprite2d.model.clone()));
		let transform = Box::new(Transform2D::default());

		node.add_component(sprite2d);
		node.add_component(model);
		node.add_component(transform);
		node
	}

	pub fn texture(&self) -> ResourceRef<ResTexture2D> { self.texture.clone() }
	pub fn set_texture(&mut self, texture: ResourceRef<ResTexture2D>) {
		self.texture = texture.clone();

		let mut model = Box::new(UNIT_QUAD.clone());
		model.meshes[0].material.map_0 = texture.borrow().get().handle();

		self.model = Rc::new(model);

		if self.owner == 0 {
			return;
		}

		let scene = self.scene.clone().unwrap();

		if let Ok(mut scene) = scene.write() {
			let owner = scene.node_mut(self.owner).unwrap();

			if let Some(model) = owner.component_mut::<components::Model>() {
				model.set_model(self.model.clone());
			} else {
				log::warn!("Node {} has Sprite2D, but not Model", owner.id());
			}
		}
	}
}

// impl NodeComponent for Sprite2D {
// 	fn enter_scene(&mut self, owner: NodeId, scene: Arc<Mutex<SceneGraph>>) {
// 		if let Ok(mut scene) = scene.clone().lock() {
// 			let owner = scene.node_mut(owner).unwrap();
// 			owner.add_component(Box::new(components::Model::new(self.model.clone())));
// 		}

// 		self.owner = owner;
// 		self.scene = Some(scene);
// 	}

// 	fn exit_scene(&mut self) {
// 		self.owner = Default::default();
// 		self.scene = Default::default();
// 	}

// 	fn as_any(&self) -> &dyn std::any::Any {
// 		self
// 	}

// 	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
// 		self
// 	}
// }
