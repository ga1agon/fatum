use glam::{EulerRot, Mat4, Quat, Vec2, Vec3, Vec4};

pub trait Transform {
	fn global_matrix(&self) -> Mat4;
	fn set_global_matrix(&mut self, matrix: Mat4);

	fn local_matrix(&self) -> Mat4;
	fn set_local_matrix(&mut self, matrix: Mat4);

	fn dirty(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3D {
	local_matrix: Mat4,
	global_matrix: Mat4,

	translation: Vec3,
	rotation: Quat,
	scale: Vec3,

	dirty: bool
}

impl Transform3D {
	pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
		Self {
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation,
			rotation,
			scale,
			dirty: true
		}
	}

	pub fn with_euler(translation: Vec3, order: EulerRot, rotation: Vec3, scale: Vec3) -> Self {
		Self {
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation,
			rotation: Quat::from_euler(order, rotation.x, rotation.y, rotation.z),
			scale,
			dirty: true
		}
	}

	pub fn with_translation(translation: Vec3) -> Self {
		Self {
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation,
			rotation: Quat::IDENTITY,
			scale: Vec3::ONE,
			dirty: true
		}
	}

	pub fn translate(&mut self, translation: Vec3) {
		self.translation += translation;
	}

	pub fn rotate(&mut self, rotation: Quat) {
		self.rotation += rotation;
	}

	pub fn rotate_euler(&mut self, order: EulerRot, rotation: Vec3) {
		self.rotation += Quat::from_euler(order, rotation.x, rotation.y, rotation.z)
	}

	// pub fn scale(&mut self, scale: Vec3) {
	// 	self.scale += scale;
	// }

	pub fn look_at(&mut self, target: Vec3) {
		let forward = (target - self.translation).normalize();

		if forward == Vec3::ZERO {
			self.rotation = Quat::IDENTITY;
			return;
		}

		let right = Vec3::Y.cross(forward).normalize();
		let up = forward.cross(right);

		let matrix = Mat4::from_cols(
			Vec4::new(right.x, right.y, right.z, 0.0), 
			Vec4::new(up.x, up.y, up.z, 0.0), 
			Vec4::new(forward.x, forward.y, forward.z, 0.0), 
			Vec4::new(0.0, 0.0, 0.0, 1.0)
		);

		self.rotation = Quat::from_mat4(&matrix);
	}

	pub fn translation(&self) -> Vec3 { self.translation }
	pub fn set_translation(&mut self, translation: Vec3) { self.translation = translation; }

	pub fn rotation(&self) -> Quat { self.rotation }
	pub fn set_rotation(&mut self, rotation: Quat) { self.rotation = rotation }

	pub fn rotation_euler(&self, order: EulerRot) -> Vec3 { self.rotation.to_euler(order).into() }
	pub fn set_rotation_euler(&mut self, order: EulerRot, rotation: Vec3) { self.rotation = Quat::from_euler(order, rotation.x, rotation.y, rotation.z) }

	pub fn scale(&self) -> Vec3 { self.scale }
	pub fn set_scale(&mut self, scale: Vec3) { self.scale = scale }
}

impl Transform for Transform3D {
	fn global_matrix(&self) -> Mat4 { self.global_matrix }
	fn set_global_matrix(&mut self, matrix: Mat4) { self.global_matrix = matrix }

	fn local_matrix(&self) -> Mat4 { self.local_matrix }
	fn set_local_matrix(&mut self, matrix: Mat4) { self.local_matrix = matrix }

	fn dirty(&self) -> bool { self.dirty }
}

impl Default for Transform3D {
	fn default() -> Self {
		Self {
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation: Vec3::ZERO,
			rotation: Quat::IDENTITY,
			scale: Vec3::ONE,
			dirty: true
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2D {
	local_matrix: Mat4,
	global_matrix: Mat4,

	translation: Vec2,
	rotation: Vec2,
	scale: Vec2,

	dirty: bool
}

impl Transform2D {
	pub fn new(translation: Vec2, rotation: Vec2, scale: Vec2) -> Self {
		Self {
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation,
			rotation,
			scale,
			dirty: true
		}
	}

	pub fn with_translation(translation: Vec2) -> Self {
		Self {
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation,
			rotation: Vec2::ZERO,
			scale: Vec2::ONE,
			dirty: true
		}
	}

	pub fn translate(&mut self, translation: Vec2) {
		self.translation += translation;
	}

	pub fn rotate(&mut self, rotation: Vec2) {
		self.rotation += rotation;
	}

	// pub fn scale(&mut self, scale: Vec3) {
	// 	self.scale += scale;
	// }

	pub fn translation(&self) -> Vec2 { self.translation }
	pub fn set_translation(&mut self, translation: Vec2) { self.translation = translation; }

	pub fn rotation(&self) -> Vec2 { self.rotation }
	pub fn set_rotation(&mut self, rotation: Vec2) { self.rotation = rotation }

	pub fn scale(&self) -> Vec2 { self.scale }
	pub fn set_scale(&mut self, scale: Vec2) { self.scale = scale }
}

impl Transform for Transform2D {
	fn global_matrix(&self) -> Mat4 { self.global_matrix }
	fn set_global_matrix(&mut self, matrix: Mat4) { self.global_matrix = matrix }

	fn local_matrix(&self) -> Mat4 { self.local_matrix }
	fn set_local_matrix(&mut self, matrix: Mat4) { self.local_matrix = matrix }

	fn dirty(&self) -> bool { self.dirty }
}

impl Default for Transform2D {
	fn default() -> Self {
		Self {
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation: Vec2::ZERO,
			rotation: Vec2::ZERO,
			scale: Vec2::ONE,
			dirty: true
		}
	}
}
