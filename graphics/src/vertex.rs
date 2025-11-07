use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3, Vec3A};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Pod, Zeroable, PartialEq)]
pub struct Vertex {
	pub position: Vec3A,
	//#[doc(hidden)] _padding0: f32,
	pub normal: Vec3A,
	//#[doc(hidden)] _padding1: f32,
	pub tangent: Vec3A,
	//#[doc(hidden)] _padding2: f32,
	pub bitangent: Vec3A,
	//#[doc(hidden)] _padding3: f32,
	pub uv: Vec2,
	#[doc(hidden)] padding0: Vec2
}

impl Vertex {
	pub fn new(
		position: Vec3,
		normal: Vec3,
		tangent: Vec3,
		bitangent: Vec3,
		uv: Vec2
	) -> Self {
		Self {
			position: position.into(),
			normal: normal.into(),
			tangent: tangent.into(),
			bitangent: bitangent.into(),
			uv,
			padding0: Default::default()
		}
	}
}
