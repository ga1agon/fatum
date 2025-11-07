use glfw::{Context, PWindow};
use glow::{HasContext, NativeBuffer, NativeVertexArray};

use crate::{Model, Vertex, platform::{GraphicsContext, opengl::{OpenGlContext, OpenGlWindow}}, render::*};
use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc, time};

struct ObjectDrawData {
	id: u64,
	gl: Rc<glow::Context>,
	vaos: Vec<NativeVertexArray>,
	vbos: Vec<NativeBuffer>,
}

impl Drop for ObjectDrawData {
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
	targets: Vec<Box<dyn RenderTarget>>,
	commands: Vec<fn(time::Duration)>,

	last_process: time::Instant,
	process_delta: time::Duration,

	objects: HashMap<Rc<RenderObject>, ObjectDrawData>,
}

impl OpenGlRenderQueue {
	pub fn new(context: Rc<OpenGlContext>) -> Self {
		Self {
			context,
			pipeline: None,
			targets: vec![],
			commands: vec![],
			last_process: std::time::Instant::now(),
			process_delta: std::time::Duration::from_secs(0),
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

		for target in &mut self.targets {
			target.begin();

			for (object, draw_data) in &self.objects {
				let meshes = &object.model.meshes;

				pipeline.matrix_data().set_data(vec![object.matrix].into());
				pipeline.matrix_data().push();

				let vaos = &draw_data.vaos;

				for i in 0..object.model.meshes.len() {
					let mesh = &meshes[i];

					pipeline.material_data().set_data(vec![mesh.material].into());
					pipeline.material_data().push();

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

			for command in &self.commands {
				command(self.process_delta);
			}

			target.end();
		}

		pipeline.end();
	}

	fn pipeline(&self) -> &Option<Box<dyn RenderPipeline>> { &self.pipeline }
	fn set_pipeline(&mut self, pipeline: Option<Box<dyn RenderPipeline>>) { self.pipeline = pipeline }

	fn add_target(&mut self, target: Box<dyn RenderTarget>) {
		self.targets.push(target);
	}
	
	fn add_command(&mut self, command: fn(time::Duration)) {
		self.commands.push(command);
	}
	
	fn add_object(&mut self, object: Rc<RenderObject>) -> bool {
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
			ObjectDrawData {
				id: object.id,
				gl: gl.clone(),
				vaos,
				vbos
			}
		);

		true
	}
	
	fn remove_object(&mut self, object: &RenderObject) -> bool {
		self.objects.remove(object).is_some()
	}
}
