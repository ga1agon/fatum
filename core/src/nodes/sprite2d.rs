use std::{cell::{LazyCell, OnceCell, RefCell}, rc::Rc, sync::{Arc, Mutex}};

use fatum_graphics::{Color, Material, Mesh, Model, Vertex, render::RenderObject, texture::Texture2D};
use fatum_macros::node_impl_new;
use fatum_scene::{Node, NodeBehaviour, NodeComponent, NodeId, SceneGraph};
use glam::{Vec2, Vec3};
use lazy_static::lazy_static;
use static_init::dynamic;
use crate::{nodes::Node2D, resources::ResTexture2D};

// #[dynamic]
// static UNIT_QUAD: Model = Model {
// 	meshes: vec![
// 		Mesh {
// 			vertices: vec![
// 				Vertex::new(Vec3::new(0.0, 0.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(0.0, 0.0)),
// 				Vertex::new(Vec3::new(0.0, 200.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(0.0, 1.0)),
// 				Vertex::new(Vec3::new(200.0, 200.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(1.0, 1.0)),
// 				Vertex::new(Vec3::new(200.0, 0.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(1.0, 0.0)),
// 			],
// 			indices: vec![
// 				0u32, 1u32, 2u32,
// 				0u32, 2u32, 3u32
// 			],
// 			material: Material::default()
// 		}
// 	]
// };

// pub fn Sprite2D(sprite2d: Box<Sprite2D>) -> Node {
// 	let mut node = Node2D();

// 	let mut model = Box::new(UNIT_QUAD.clone());
// 	model.meshes[0].material.map_0 = sprite2d.texture.borrow().get().handle() as u32;
	
// 	node.add_component(sprite2d);
// 	node
// }

// impl ObjectRenderable for Sprite2D {

// }

// pub struct Sprite2D {
// 	pub texture: Rc<RefCell<Box<ResTexture2D>>>,
// }

// impl Node for Sprite2D {
// 	fn id(&self) -> u32 {
// 		todo!()
// 	}

// 	fn name(&self) -> &str {
// 		todo!()
// 	}

// 	fn set_name(&mut self, name: &str) {
// 		todo!()
// 	}

// 	fn scene(&self) -> Option<std::sync::Arc<std::sync::Mutex<fatum_scene::SceneGraph>>> {
// 		todo!()
// 	}

// 	fn parent(&self) -> u32 {
// 		todo!()
// 	}

// 	fn children(&self) -> Vec<u32> {
// 		todo!()
// 	}

// 	fn enter_scene(&mut self, id: u32, scene: std::sync::Arc<std::sync::Mutex<fatum_scene::SceneGraph>>) {
// 		let mut scene = scene.lock().unwrap();
// 		scene.add_behaviour::<Sprite2D, Renderable>(id, self);
// 	}

// 	fn exit_scene(&mut self) {
// 		todo!()
// 	}
	
// 	fn as_any(&self) -> &dyn std::any::Any { self }
// }

// impl Renderable for Sprite2D {
// 	fn object(&self) -> RenderObject {
// 		todo!()
// 	}

// 	fn render(&self, node: &Box<dyn Node>, delta: std::time::Duration) {
// 		todo!()
// 	}
// }

// impl NodeBehaviour for Sprite2D {}

// #[derive(NodeBehaviour)]
// pub struct Sprite2D {
// 	pub base: Node2D,
// 	pub object: Rc<RenderObject>
// }

// impl Sprite2D {
// 	pub fn new(texture: Rc<RefCell<Box<ResTexture2D>>>) -> Self {
// 		let UNIT_QUAD: Model = Model {
// 			meshes: vec![
// 				Mesh {
// 					vertices: vec![
// 						Vertex::new(Vec3::new(0.0, 0.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(0.0, 0.0)),
// 						Vertex::new(Vec3::new(0.0, 1.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(0.0, 1.0)),
// 						Vertex::new(Vec3::new(1.0, 1.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(1.0, 1.0)),
// 						Vertex::new(Vec3::new(1.0, 0.0, 0.0), Default::default(), Default::default(), Default::default(), Vec2::new(1.0, 0.0)),
// 					],
// 					indices: vec![
// 						0u32, 1u32, 2u32,
// 						0u32, 2u32, 3u32
// 					],
// 					material: Material::default()
// 				}
// 			]
// 		};

// 		let base = Node2D::new();

// 		let material = Material::with_textures_pbr(
// 			Color::from_rgb_u8(255, 255, 255),
// 			0.0,
// 			1.0,
// 			1.5,
// 			Some(texture.borrow().get()),
// 			None,
// 			None,
// 			None,
// 			None
// 		);

// 		let mut model = UNIT_QUAD.clone();
// 		model.meshes[0].material = material;

// 		let translation = Vec3::new(0.0, 0.0, 0.0);
// 		let rotation = glam::Quat::from_euler(glam::EulerRot::YXZ, 0.0, 0.0, 0.0);
// 		let scale = Vec3::new(1.0, 1.0, 1.0);
// 		let matrix = glam::Mat4::from_scale_rotation_translation(scale, rotation, translation);

// 		let object = RenderObject::new(model, matrix);

// 		Self {
// 			base,
// 			object: object.into()
// 		}
// 	}
// }

// impl Into<Node> for Sprite2D {
// 	fn into(self) -> Node {
// 		self.base.into()
// 	}
// }
