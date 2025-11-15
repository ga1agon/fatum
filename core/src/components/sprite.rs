use std::{cell::{LazyCell, OnceCell, RefCell}, rc::Rc, sync::{Arc, Mutex}};

use fatum_graphics::{Color, Material, Mesh, Model, Vertex, render::RenderObject, texture::Texture2D};
use fatum_macros::node_impl_new;
use fatum_resources::ResourceRef;
use fatum_scene::{Node, NodeComponent, NodeId, SceneGraph, SharedSceneGraph};
use glam::{Vec2, Vec3};
use static_init::dynamic;
use crate::{components::{self, Transform2D}, resources::ResTexture2D};

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

#[derive(NodeComponent, Clone)]
pub struct Sprite {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,
	texture: ResourceRef<ResTexture2D>,
	pub(crate) model: Rc<Box<fatum_graphics::Model>>
}

impl Sprite {
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
				log::warn!("Node {} has Sprite, but not Model", owner.id());
			}
		}
	}
}
