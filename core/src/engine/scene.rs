use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, Mutex, RwLockWriteGuard}};

use fatum_graphics::{Camera, platform::GraphicsPlatform, render::{RenderObject, RenderQueue}};
use fatum_resources::ResourcePlatform;
use fatum_scene::{Node, NodeId, SceneGraph, SharedSceneGraph, iterators::{SceneDfsIterator, ScenePostDfsIterator}};
use fatum_signals::SignalDispatcher;
use glam::{Mat4, Quat, Vec3, Vec4};
use signals2::Connect2;

use crate::{Application, CoreEngine, GraphicsEngine, components::{self, Model, Transform, Transform2D, Transform3D}};

pub struct SceneEngine<P: GraphicsPlatform> {
	graphics: Rc<RefCell<GraphicsEngine<P>>>,
	scenes: HashMap<usize, SharedSceneGraph>,
}

impl<P> SceneEngine<P> where P: GraphicsPlatform {
	pub fn new(graphics: Rc<RefCell<GraphicsEngine<P>>>) -> Self {
		log::info!("Created scene engine");

		Self {
			graphics,
			scenes: HashMap::new()
		}
	}

	pub fn scene(&self, output_index: usize) -> Option<SharedSceneGraph> {
		self.scenes.get(&output_index).map_or(None, |v| Some(v.clone()))
	}

	pub fn set_scene(&mut self, output_index: usize, scene: SharedSceneGraph) -> Option<bool> {
		log::info!("Setting scene for output {}: {:?}", output_index, scene);

		let mut graphics = self.graphics.borrow_mut();

		let queue = graphics.output(output_index)?;

		{
			let nodes: Vec<u32> = ScenePostDfsIterator::new(scene.clone(), Default::default())
				.collect();

			let mut scene = scene.write().unwrap();

			for node in nodes {
				let node = scene.node_mut(node)
					.expect("An invalid node was provided by the post-order DFS traverse iterator?");

				if let Some(model) = node.component::<Model>() {
					let render_object: RenderObject = model.into();
					queue.add_object(&render_object, Mat4::IDENTITY);
				}

				// :3
				node.component_added.connect_capture(vec![queue as *mut _ as *mut std::ffi::c_void], |captures, args| {
					unsafe {
						let queue = &mut *(captures[0] as *mut Box<dyn RenderQueue>);
						let component = &*args.1;

						if let Some(model) = component.as_any().downcast_ref::<Model>() {
							let render_object: RenderObject = model.into();
							queue.add_object(&render_object, Mat4::IDENTITY);
						}
					}
				});

				node.component_removed.connect_capture(vec![queue as *mut _ as *mut std::ffi::c_void], |captures, args| {
					unsafe {
						let queue = &mut *(captures[0] as *mut Box<dyn RenderQueue>);
						let component = &*args.1;

						if let Some(model) = component.as_any().downcast_ref::<Model>() {
							let render_object: RenderObject = model.into();
							queue.remove_object(&render_object);
						}
					}
				});

				node.ready();
			}
		}

		{
			let mut scene = scene.write().unwrap();

			scene.node_added.connect_capture(vec![queue as *mut _ as *mut std::ffi::c_void], |captures, args| {
				unsafe {
					let queue = &mut *(captures[0] as *mut Box<dyn RenderQueue>);
					let node = &*args.1;

					if let Some(model) = node.component::<Model>() {
						let render_object: RenderObject = model.into();
						queue.add_object(&render_object, Mat4::IDENTITY);
					}

					node.ready();
				}
			});

			scene.node_removed.connect_capture(vec![queue as *mut _ as *mut std::ffi::c_void], |captures, args| {
				unsafe {
					let queue = &mut *(captures[0] as *mut Box<dyn RenderQueue>);
					let node = &*args.1;

					if let Some(model) = node.component::<Model>() {
						let render_object: RenderObject = model.into();
						queue.remove_object(&render_object);
					}
				}
			});
		}

		self.scenes.insert(output_index, scene);
		log::info!("Scene imported for output {}", output_index);
		Some(true)
	}

	pub fn process(&mut self, delta: std::time::Duration) -> bool {
		for (output, scene) in &self.scenes {
			if let Some(queue) = self.graphics.borrow_mut().output(*output) {
				let nodes: Vec<u32> = SceneDfsIterator::new(scene.clone(), Default::default())
					.collect();

				let mut camera_data: Option<fatum_graphics::Camera> = None;
				let mut matrix_delta: HashMap<NodeId, (Mat4, Mat4)> = HashMap::new();

				if let Ok(scene) = scene.try_read() {
					for node in &nodes {
						let node = scene.node(*node)
							.expect("Iterator returned a non-existing node");

						if node.id() == Default::default() {
							continue; // ignore root
						}

						node.emit("update", delta);

						if camera_data.is_none() && let Some(c) = node.component::<components::Camera>() {
							if c.is_active() {
								camera_data = Some(c.into());
							}
						}

						let parent = node.parent();

						let (parent_dirty, parent_global_matrix) =
							if let Some((_, global_matrix)) = matrix_delta.get(&parent) {
								(true, *global_matrix)
							} else if let Some(parent) = scene.node(parent) {
								if let Some(parent_t2d) = parent.component::<Transform2D>() {
									(parent_t2d.dirty, parent_t2d.global_matrix)
								} else if let Some(parent_t3d) = parent.component::<Transform3D>() {
									(parent_t3d.dirty, parent_t3d.global_matrix)
								} else {
									(false, Mat4::IDENTITY)
								}
							} else {
								(false, Mat4::IDENTITY)
							};

						if let Some(t2d) = node.component::<Transform2D>() {
							// node is dirty if it itself is dirty OR its parent is dirty
							let dirty = parent_dirty | t2d.dirty;

							if !dirty {
								continue;
							}

							let local_matrix = t2d.calculate_matrix();
							let global_matrix = parent_global_matrix * local_matrix;

							matrix_delta.insert(node.id(), (local_matrix, global_matrix));
						} else if let Some(t3d) = node.component::<Transform3D>() {
							let dirty = parent_dirty | t3d.dirty;

							if !dirty {
								continue;
							}

							let local_matrix = t3d.calculate_matrix();
							let global_matrix = parent_global_matrix * local_matrix;

							matrix_delta.insert(node.id(), (local_matrix, global_matrix));
						}
					}
				} else {
					log::warn!("Cannot process scene: could not get a read lock");
					continue;
				}

				if let Ok(mut scene) = scene.try_write() {
					for node in &nodes {
						let node = scene.node_mut(*node).unwrap();
						node.emit_mut("$update", delta);

						if !matrix_delta.contains_key(&node.id()) {
							continue; // didn't change
						}

						let (local_matrix, global_matrix) = matrix_delta.get(&node.id()).unwrap();
						let t: &mut dyn Transform;

						if let Some(t2d) = node.component_mut::<Transform2D>() {
							t = t2d;
						} else if let Some(t3d) = node.component_mut::<Transform3D>() {
							t = t3d;
						} else {
							log::warn!("{:?} had its transform processed in the read pass, but now it doesn't have one?", node);
							continue;
						}
						
						t.set_local_matrix(*local_matrix);
						t.set_global_matrix(*global_matrix);
						t.set_dirty(false);

						if let Some(model) = node.component::<Model>() {
							let render_object: RenderObject = model.into();
							queue.set_object_matrix(&render_object, *global_matrix);
						}
					}
				} else {
					log::warn!("Cannot process scene: could not get a write lock");
				}

				// set camera data
				if let Some(camera_data) = camera_data {
					queue.pipeline_mut().unwrap()
						.camera_data().set_data(vec![camera_data].into());
				} // TODO else set some default?
			} else {
				log::warn!("Cannot process scene: could not get the render queue for output {}", output);
			}
		}
		true
	}
}
