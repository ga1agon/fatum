use std::{cell::{LazyCell, OnceCell, RefCell}, rc::Rc, sync::{Arc, Mutex}};

use fatum_graphics::{Color, Material, Mesh, Model, Vertex, render::RenderObject, texture::Texture2D};
use fatum_macros::node_impl_new;
use fatum_resources::ResourceRef;
use fatum_scene::{Node, NodeComponent, NodeId, SceneGraph, SharedSceneGraph};
use glam::{Vec2, Vec3};
use lazy_static::lazy_static;
use static_init::dynamic;
use crate::{components::{self, Sprite, Transform2D}, resources::ResTexture2D};

pub struct Sprite2D {}

impl Sprite2D {
	pub fn new(texture: ResourceRef<ResTexture2D>) -> Node {
		let mut node = Node::new();

		let sprite = Box::new(Sprite::new(texture));
		let model = Box::new(components::Model::new(sprite.model.clone()));
		let transform = Box::new(Transform2D::default());

		node.add_component(sprite);
		node.add_component(model);
		node.add_component(transform);
		node
	}
}
