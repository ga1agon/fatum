use bytemuck::{Pod, Zeroable};
use glam::{Mat4, UVec2, Vec2, Vec3, Vec4};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable, PartialEq)]
pub struct Camera {
	pub projection: Mat4,
	pub inverse_projection: Mat4,
	pub view: Mat4,
	pub inverse_view: Mat4,
	pub position: Vec3,
	pub aspect_ratio: f32
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable, PartialEq)]
pub struct Camera3D {
	pub position: Vec3,
	pub target: Vec3,
	pub up: Vec3,
	pub size: UVec2,
	pub fov: f32,
}

impl Camera3D {
	pub const FRONT: Vec3 = Vec3::Z;
	pub const UP: Vec3 = Vec3::Y;

	pub const Z_NEAR: f32 = 0.01;
	pub const Z_FAR: f32 = 1000.0;

	pub fn create_perspective(&self) -> Camera {
		let aspect_ratio = self.size.x as f32 / self.size.y as f32;

		let projection = Mat4::perspective_rh_gl(
			self.fov.to_radians(),
			aspect_ratio,
			Self::Z_NEAR,
			Self::Z_FAR
		);

		let inverse_projection = projection.inverse();

		let view = Mat4::look_at_rh(
			self.position,
			self.target,
			self.up
		);

		let inverse_view = view.inverse();

		Camera {
			projection,
			inverse_projection,
			view,
			inverse_view,
			position: self.position,
			aspect_ratio
		}
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable, PartialEq)]
pub struct Camera2D {
	pub position: Vec2,
	pub up: Vec3,
	pub size: UVec2,
}

impl Camera2D {
	pub const FRONT: Vec3 = Vec3::Z;
	pub const UP: Vec3 = Vec3::Y;

	pub fn create(&self) -> Camera {
		// TODO use self.up
		let projection = Mat4::from_cols(
			Vec4::new(2.0 / self.size.x as f32, 0.0, 0.0, 0.0), 
			Vec4::new(0.0, 2.0 / self.size.y as f32, 0.0, 0.0), 
			Vec4::new(0.0, 0.0, 1.0, 0.0),
			Vec4::new(-1.0, -1.0, 0.0, 1.0)
		);

		let inverse_projection = projection.inverse();

		let view = Mat4::from_cols(
			Vec4::new(1.0, 0.0, 0.0, 0.0), 
			Vec4::new(0.0, 1.0, 0.0, 0.0), 
			Vec4::new(0.0, 0.0, 1.0, 0.0),
			Vec4::new(-self.position.x, -self.position.y, 0.0, 1.0)
		);

		let inverse_view = view.inverse();

		let position = Vec3::new(self.position.x, self.position.y, 0.0);
		let aspect_ratio = self.size.x as f32 / self.size.y as f32;

		Camera {
			projection,
			inverse_projection,
			view,
			inverse_view,
			position,
			aspect_ratio
		}
	}
}
