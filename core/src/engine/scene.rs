use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, Mutex, RwLockWriteGuard}};

use fatum_graphics::{Camera, platform::GraphicsPlatform, render::{RenderObject, RenderQueue}};
use fatum_resources::ResourcePlatform;
use fatum_scene::{Node, NodeBehaviour, NodeId, SceneGraph, SharedSceneGraph, iterators::{SceneDfsIterator, ScenePostDfsIterator}};
use glam::{Mat4, Quat, Vec3};
use signals2::Connect2;

use crate::{Application, CoreEngine, GraphicsEngine, components::{Camera2D, Model, Transform, Transform2D, Transform3D}};

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

		let queue = graphics.get_output(output_index)?;

		{
			let nodes: Vec<u32> = ScenePostDfsIterator::new(scene.clone(), Default::default())
				.collect();

			let mut scene = scene.write().unwrap();

			for node in nodes {
				let node = scene.node_mut(node)
					.expect("An invalid node was provided by the post-order DFS traverse iterator?");

				if let Some(model) = node.component::<Model>()
					&& let Some(transform2d) = node.component::<Transform2D>()
				{
					let render_object: RenderObject = model.into();
					queue.add_object(&render_object, transform2d.global_matrix);
				}

				if let Some(model) = node.component::<Model>()
					&& let Some(transform3d) = node.component::<Transform3D>()
				{
					let render_object: RenderObject = model.into();
					queue.add_object(&render_object, transform3d.global_matrix);
				}

				// TODO on component added/removed model

				// node ready
			}
		}

		{
			let mut scene = scene.write().unwrap();

			scene.node_added.connect(|(scene, node)| {
				// TODO add to queue
			});

			scene.node_removed.connect(|(scene, node)| {
				// TODO remove from queue
			});
		}

		self.scenes.insert(output_index, scene);
		log::info!("Scene imported for output {}", output_index);
		Some(true)
	}

	pub fn process(&mut self) -> bool {
		for (output, scene) in &self.scenes {
			if let Some(queue) = self.graphics.borrow_mut().get_output(*output) {
				let nodes: Vec<u32> = SceneDfsIterator::new(scene.clone(), Default::default())
					.collect();

				let mut camera_data: Option<fatum_graphics::Camera> = None;
				let mut matrix_delta: HashMap<NodeId, (Mat4, Mat4)> = HashMap::new();

				if let Ok(scene) = scene.try_read() {
					let mut prev_dirty = false;
					
					for node in &nodes {
						let node = scene.node(*node)
							.expect("Iterator returned a non-existing node");

						if node.id() == Default::default() {
							continue; // ignore root
						}

						if camera_data.is_none() && let Some(c2d) = node.component::<Camera2D>() {
							if c2d.is_active() {
								camera_data = Some(c2d.camera_data());
							}
						}

						let parent = node.parent();
						let mut dirty = prev_dirty;

						if let Some(t2d) = node.component::<Transform2D>() {
							// node is dirty if it itself is dirty OR its parent is dirty
							dirty |= t2d.dirty;

							if !dirty {
								//log::debug!("{}: skipping, not dirty", node.id());
								continue;
							}

							let scale = t2d.scale();
							let rotation = t2d.rotation();
							let translation = t2d.translation();

							let local_matrix = Mat4::from_scale_rotation_translation(
								Vec3::new(scale.x, scale.y, 1.0),
								Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, 0.0),
								Vec3::new(translation.x, translation.y, 0.0)
							);

							let parent_global_matrix: Mat4;

							if let Some((_, global_matrix)) = matrix_delta.get(&parent) {
								//log::debug!("{}: Using parent global matrix from delta", node.id());
								parent_global_matrix = *global_matrix;
							} else if let Some(parent) = scene.node(parent)
								&& let Some(parent_t2d) = parent.component::<Transform2D>()
							{
								//log::debug!("{}: Using parent global matrix from component", node.id());
								parent_global_matrix = parent_t2d.global_matrix;
							} else {
								//log::debug!("{}: Parent global matrix identity", node.id());
								parent_global_matrix = Mat4::IDENTITY;
							}

							let global_matrix = local_matrix * parent_global_matrix;

							matrix_delta.insert(node.id(), (local_matrix, global_matrix));
						}

						prev_dirty = dirty;
					}
				} else {
					log::warn!("Cannot process scene: could not get a read lock");
					continue;
				}

				if let Ok(mut scene) = scene.try_write() {
					for node in &nodes {
						if !matrix_delta.contains_key(node) {
							continue; // didn't change
						}

						let (local_matrix, global_matrix) = matrix_delta.get(node).unwrap();
						
						let node = scene.node_mut(*node).unwrap();
						let t2d = node.component_mut::<Transform2D>().unwrap();

						t2d.local_matrix = *local_matrix;
						t2d.global_matrix = *global_matrix;
						t2d.dirty = false;

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
