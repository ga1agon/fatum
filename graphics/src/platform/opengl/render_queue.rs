use glam::Mat4;
use glfw::{Context, PWindow};
use glow::{HasContext, NativeBuffer, NativeTexture, NativeVertexArray};

use crate::{Camera, Model, Rf, Vertex, platform::{GraphicsContext, opengl::{OpenGlContext, OpenGlWindow}}, render::*};
use std::{cell::RefCell, collections::HashMap, hash::Hash, num::{NonZero, NonZeroU32}, rc::Rc, sync::atomic::{AtomicUsize, Ordering}, time};

struct ObjectData {
	id: u64,
	matrix: Mat4,

	gl: Rc<glow::Context>,
	vaos: Vec<NativeVertexArray>,
	vbos: Vec<NativeBuffer>,
}

impl Drop for ObjectData {
	fn drop(&mut self) {
		for vao in &self.vaos {
			unsafe {
				self.gl.delete_vertex_array(*vao);
			}
		}

		for vbo in &self.vbos {
			unsafe {
				self.gl.delete_buffer(*vbo);
			}
		}
	}
}

pub struct OpenGlRenderQueue {
	context: Rc<OpenGlContext>,

	pipeline: Option<Box<dyn RenderPipeline>>,
	targets: HashMap<usize, Box<dyn RenderTarget>>,
	commands: HashMap<usize, fn(time::Duration)>,

	last_process: time::Instant,
	process_delta: time::Duration,

	objects: HashMap<RenderObject, ObjectData>,
}

impl OpenGlRenderQueue {
	const ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

	pub fn new(context: Rc<OpenGlContext>) -> Self {
		Self {
			context,
			pipeline: None,
			targets: HashMap::new(),
			commands: HashMap::new(),
			last_process: time::Instant::now(),
			process_delta: time::Duration::from_secs(0),
			objects: HashMap::new()
		}
	}
}

impl RenderQueue for OpenGlRenderQueue {
	fn process(&mut self) {
		if self.pipeline.is_none() {
			return
		}

		let now = time::Instant::now();
		self.process_delta = now - self.last_process;
		self.last_process = now;

		let pipeline = self.pipeline.as_mut().unwrap();
		pipeline.begin();

		let gl = self.context.get();

		for (_, target) in &mut self.targets {
			target.begin();

			for (object, data) in &self.objects {
				let meshes = &object.model.meshes;

				pipeline.matrix_data().set_data(vec![data.matrix].into());
				pipeline.matrix_data().push();

				let vaos = &data.vaos;

				for i in 0..object.model.meshes.len() {
					let mesh = &meshes[i];
					let material = mesh.material;

					pipeline.material_data().set_data(vec![material].into());
					pipeline.material_data().push();

					// ugly as shit
					unsafe {
						if material.map_0 > 0 {
							gl.active_texture(glow::TEXTURE0);
							gl.bind_texture(glow::TEXTURE_2D, Some(NativeTexture(NonZero::<u32>::new_unchecked(material.map_0))));
						}

						if material.map_1 > 0 {
							gl.active_texture(glow::TEXTURE1);
							gl.bind_texture(glow::TEXTURE_2D, Some(NativeTexture(NonZero::<u32>::new_unchecked(material.map_1))));
						}

						if material.map_2 > 0 {
							gl.active_texture(glow::TEXTURE2);
							gl.bind_texture(glow::TEXTURE_2D, Some(NativeTexture(NonZero::<u32>::new_unchecked(material.map_2))));
						}

						if material.map_3 > 0 {
							gl.active_texture(glow::TEXTURE3);
							gl.bind_texture(glow::TEXTURE_2D, Some(NativeTexture(NonZero::<u32>::new_unchecked(material.map_3))));
						}

						if material.map_4 > 0 {
							gl.active_texture(glow::TEXTURE4);
							gl.bind_texture(glow::TEXTURE_2D, Some(NativeTexture(NonZero::<u32>::new_unchecked(material.map_4))));
						}
					}

					unsafe {
						gl.bind_vertex_array(Some(vaos[i]));

						gl.draw_elements(
							glow::TRIANGLES,
							mesh.indices.len() as i32,
							glow::UNSIGNED_INT,
							0
						);
					}
				}
			}

			for (_, command) in &self.commands {
				command(self.process_delta);
			}

			target.end();
		}

		pipeline.end();
	}

	fn is_active(&self) -> bool {
		self.targets.iter().all(|t| t.1.is_active())
	}

	fn pipeline(&self) -> Option<&Box<dyn RenderPipeline>> { self.pipeline.as_ref() }
	fn pipeline_mut(&mut self) -> Option<&mut Box<dyn RenderPipeline>> { self.pipeline.as_mut() }
	fn set_pipeline(&mut self, pipeline: Option<Box<dyn RenderPipeline>>) { self.pipeline = pipeline }

	fn targets(&self) -> Vec<usize> {
		let mut ids = Vec::new();

		for id in self.targets.keys() {
			ids.push(*id);
		}

		ids
	}

	fn add_target(&mut self, target: Box<dyn RenderTarget>) -> usize {
		let id = Self::ID_COUNTER.fetch_add(1, Ordering::Relaxed);

		self.targets.insert(id, target);
		id
	}

	fn get_target(&self, index: usize) -> Option<&Box<dyn RenderTarget>> {
		if !self.targets.contains_key(&index) {
			return None;
		}

		Some(&self.targets[&index])
	}

	fn get_target_mut(&mut self, index: usize) -> Option<&mut Box<dyn RenderTarget>> {
		if !self.targets.contains_key(&index) {
			return None;
		}

		self.targets.get_mut(&index)
	}

	fn remove_target(&mut self, index: usize) -> bool {
		self.targets.remove(&index).is_some()
	}
	
	fn add_command(&mut self, command: fn(time::Duration)) -> usize {
		let id = Self::ID_COUNTER.fetch_add(1, Ordering::Relaxed);

		self.commands.insert(id, command);
		id
	}

	fn remove_command(&mut self, index: usize) -> bool {
		self.targets.remove(&index).is_some()
	}
	
	fn add_object(&mut self, object: &RenderObject, matrix: Mat4) -> bool {
		let gl = self.context.get();

		let meshes = &object.model.meshes;

		let mut vaos = Vec::with_capacity(meshes.len());
		let mut vbos = Vec::with_capacity(meshes.len() * 2);

		unsafe {
			for _ in 0..vaos.capacity() {
				let vao = gl.create_vertex_array();

				if vao.is_err() {
					return false;
				}

				vaos.push(vao.unwrap());
			}

			for _ in 0..vbos.capacity() {
				let vbo = gl.create_buffer();

				if vbo.is_err() {
					return false;
				}

				vbos.push(vbo.unwrap());
			}
		}

		for i in 0..meshes.len() {
			let mesh = &meshes[i];
			let vao = vaos[i];
			let vbo = vbos[2 * i];
			let ebo = vbos[2 * i + 1];

			assert_ne!(vao.0.get(), 0);
			assert_ne!(vbo.0.get(), 0);
			assert_ne!(ebo.0.get(), 0);

			unsafe {
				gl.bind_vertex_array(Some(vao));

				gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
				gl.buffer_data_u8_slice(
					glow::ARRAY_BUFFER,
					bytemuck::cast_slice(&mesh.vertices),
					glow::STATIC_DRAW
				);

				gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
				gl.buffer_data_u8_slice(
					glow::ELEMENT_ARRAY_BUFFER,
					bytemuck::cast_slice(&mesh.indices),
					glow::STATIC_DRAW
				);

				// vertex data
				let size = size_of::<Vertex>() as i32;

				// vec3 position
				gl.enable_vertex_attrib_array(0);
				gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, size, 0);

				// vec3 normal
				gl.enable_vertex_attrib_array(1);
				gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, size, 16);

				// vec3 tangent
				gl.enable_vertex_attrib_array(2);
				gl.vertex_attrib_pointer_f32(2, 3, glow::FLOAT, false, size, 32);

				// vec3 bitangent
				gl.enable_vertex_attrib_array(3);
				gl.vertex_attrib_pointer_f32(3, 3, glow::FLOAT, false, size, 48);

				// vec2 uv
				gl.enable_vertex_attrib_array(4);
				gl.vertex_attrib_pointer_f32(4, 2, glow::FLOAT, false, size, 64);
			}
		}

		self.objects.insert(
			object.clone(),
			ObjectData {
				id: object.id,
				matrix,
				gl: gl.clone(),
				vaos,
				vbos
			}
		);

		true
	}

	fn set_object_matrix(&mut self, object: &RenderObject, matrix: Mat4) -> bool {
		if let Some(data) = self.objects.get_mut(object) {
			data.matrix = matrix;
			return true;
		}

		false
	}
	
	fn remove_object(&mut self, object: &RenderObject) -> bool {
		self.objects.remove(object).is_some()
	}

	fn clear_objects(&mut self) {
		self.objects.clear();
	}
}
