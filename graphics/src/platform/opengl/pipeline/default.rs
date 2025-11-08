use std::{any::Any, rc::Rc};

use glam::Mat4;
use glow::HasContext;

use crate::{Camera, Color, Material, Vertex, platform::{GraphicsContext, GraphicsPlatform, opengl::{OpenGlContext, OpenGlPlatform}}, render::RenderPipeline, shader::{ShaderData, ShaderFamily, ShaderProgram}};

pub struct DefaultOpenGlPipeline {
	gl: Rc<glow::Context>,
	program: Box<dyn ShaderProgram>,

	material_data: Box<dyn ShaderData<Material>>,
	matrix_data: Box<dyn ShaderData<Mat4>>,
	camera_data: Box<dyn ShaderData<Camera>>,

	clear_color: Color,
}

impl DefaultOpenGlPipeline {
	pub fn new(context: &OpenGlContext, platform: &OpenGlPlatform) -> Self {
		let gl = context.get();
		
		const VERT_SHADER_SRC: &str =
"
#version 330 core

//= data
layout (location = 0) in vec3 v_position;
layout (location = 1) in vec3 v_normal;
layout (location = 2) in vec3 v_tangent;
layout (location = 3) in vec3 v_bitangent;
layout (location = 4) in vec2 v_uv;
//

struct Vertex {
	vec3 position;
	vec3 normal;
	vec3 tangent;
	vec3 bitangent;
	vec2 uv;
};

//= data
uniform MatrixData {
	mat4 matrix;
};

uniform CameraData {
	mat4 proj;
	mat4 inv_proj;
	mat4 view;
	mat4 inv_view;
	vec3 position;
	float aspect_ratio;
};
//

//= i/o
out Vertex vertex;
//

void main() {
	vec4 position = proj * view * matrix * vec4(v_position, 1.0);
	gl_Position = position;
	
	vertex.position = v_position;
	vertex.normal = v_normal;
	vertex.tangent = v_tangent;
	vertex.bitangent = v_bitangent;
	vertex.uv = v_uv;
}
";

		const FRAG_SHADER_SRC: &str =
"
#version 330 core

struct Vertex {
	vec3 position;
	vec3 normal;
	vec3 tangent;
	vec3 bitangent;
	vec2 uv;
};

//= data
uniform MaterialData {
	vec4 m_albedo;
	float m_roughness;
	float m_metallic;
	float m_ior;
};
//

//= i/o
in Vertex vertex;
out vec4 fragColor;
//

//= entry point
void main() {
	vec2 uv = vertex.uv;

	vec3 albedo = m_albedo.rgb;
	float opacity = 1.0;

	fragColor = vec4(albedo, opacity); // simple alpha blending
}
";

		let vert_shader = platform.create_shader(ShaderFamily::Vertex, VERT_SHADER_SRC);
		let frag_shader = platform.create_shader(ShaderFamily::Fragment, FRAG_SHADER_SRC);

		let mut program = platform.create_shader_program(vec![vert_shader, frag_shader]);
		program.build().expect("Could not build the default shader program");
		program.bind();

		let material_data = platform.create_shader_data(&program, "MaterialData", 1, None).expect("Could not create material data");
		let matrix_data = platform.create_shader_data(&program, "MatrixData", 2, None).expect("Could not create matrix data");
		let camera_data = platform.create_shader_data(&program, "CameraData", 3, None).expect("Could not create camera data");

		Self {
			gl,
			program,
			material_data,
			matrix_data,
			camera_data,
			clear_color: Color::from_rgb_f32(0.0, 0.0, 0.0)
		}
	}
}

impl RenderPipeline for DefaultOpenGlPipeline {
	fn begin(&mut self) {
		self.program.bind();

		self.material_data.push();
		self.matrix_data.push();
		self.camera_data.push();

		unsafe {
			self.gl.clear_color(
				self.clear_color.r,
				self.clear_color.g,
				self.clear_color.b,
				self.clear_color.a
			);

			self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
		}
	}
	
	fn end(&mut self) {}

	fn vertex_data(&mut self) -> &mut dyn ShaderData<Vertex> { panic!("The OpenGL backend doesn't use buffers for vertex data") }
	fn index_data(&mut self) -> &mut dyn ShaderData<u32> { panic!("The OpenGL backend doesn't use buffers for index data") }
	fn material_data(&mut self) -> &mut dyn ShaderData<Material> { self.material_data.as_mut() }
	fn matrix_data(&mut self) -> &mut dyn ShaderData<Mat4> { self.matrix_data.as_mut() }
	fn camera_data(&mut self) -> &mut dyn ShaderData<Camera> { self.camera_data.as_mut() }

	fn clear_color(&self) -> Color { self.clear_color }
	
	fn set_clear_color(&mut self, color: Color) { self.clear_color = color }
}
