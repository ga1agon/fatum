use std::rc::Rc;

use glam::Mat4;

use crate::{Camera, Color, Material, Vertex, platform::GraphicsPlatform, shader::{ShaderData, ShaderProgram}};

pub enum PipelineKind {
	Default,
	PBR,
}

pub trait RenderPipeline {
	fn begin(&mut self);
	fn end(&mut self);

	fn vertex_data(&mut self) -> &mut dyn ShaderData<Vertex>;
	fn index_data(&mut self) -> &mut dyn ShaderData<u32>;
	fn material_data(&mut self) -> &mut dyn ShaderData<Material>;
	fn matrix_data(&mut self) -> &mut dyn ShaderData<Mat4>;
	fn camera_data(&mut self) -> &mut dyn ShaderData<Camera>;

	fn clear_color(&self) -> Color;
	fn set_clear_color(&mut self, color: Color);
}
