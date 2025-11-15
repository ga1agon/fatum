use fatum_scene::{NodeComponent, NodeId, SceneGraph, SharedSceneGraph};
use glam::{EulerRot, Mat4, Quat, Vec2, Vec3, Vec4};
use std::{fmt::Debug, sync::{Arc, Mutex}};

use crate::helpers;

pub trait Transform {
	fn calculate_matrix(&self) -> Mat4;

	fn global_matrix(&self) -> Mat4;
	fn set_global_matrix(&mut self, matrix: Mat4);

	fn local_matrix(&self) -> Mat4;
	fn set_local_matrix(&mut self, matrix: Mat4);

	fn dirty(&self) -> bool;
	fn set_dirty(&mut self, dirty: bool);
}

#[derive(Clone, NodeComponent)]
pub struct Transform3D {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,

	pub(crate) local_matrix: Mat4,
	pub(crate) global_matrix: Mat4,

	translation: Vec3,
	rotation: Quat,
	scale: Vec3,

	pub(crate) dirty: bool
}

impl Transform3D {
	pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
		Self {
			owner: 0,
			scene: None,
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
			owner: 0,
			scene: None,
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
			owner: 0,
			scene: None,
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation,
			rotation: Quat::IDENTITY,
			scale: Vec3::ONE,
			dirty: true
		}
	}

	pub fn from_mat4(matrix: Mat4) -> Self {
		let (translation, rotation, scale) = helpers::mat4_decompose(matrix);
		Self::new(translation, rotation, scale)
	}

	pub fn translate(&mut self, translation: Vec3) {
		self.translation += translation;
		self.dirty = true;
	}

	pub fn rotate(&mut self, rotation: Quat) {
		self.rotation += rotation;
		self.dirty = true;
	}

	pub fn rotate_euler(&mut self, order: EulerRot, rotation: Vec3) {
		self.rotation *= Quat::from_euler(order, rotation.x, rotation.y, rotation.z);
		self.dirty = true;
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
		self.dirty = true;
	}

	pub fn translation(&self) -> Vec3 { self.translation }
	pub fn set_translation(&mut self, translation: Vec3) { self.translation = translation; self.dirty = true; }

	pub fn rotation(&self) -> Quat { self.rotation }
	pub fn set_rotation(&mut self, rotation: Quat) { self.rotation = rotation; self.dirty = true; }

	pub fn rotation_euler(&self, order: EulerRot) -> Vec3 { self.rotation.to_euler(order).into() }
	pub fn set_rotation_euler(&mut self, order: EulerRot, rotation: Vec3) { self.rotation = Quat::from_euler(order, rotation.x, rotation.y, rotation.z); self.dirty = true; }

	pub fn scale(&self) -> Vec3 { self.scale }
	pub fn set_scale(&mut self, scale: Vec3) { self.scale = scale; self.dirty = true; }
}

impl Transform for Transform3D {
	fn calculate_matrix(&self) -> Mat4 {
		let s = Mat4::from_scale(self.scale);
		let r = Mat4::from_quat(self.rotation);
		let t = Mat4::from_translation(self.translation);

		t * r * s
	}

	fn global_matrix(&self) -> Mat4 { self.global_matrix }
	fn set_global_matrix(&mut self, matrix: Mat4) { self.global_matrix = matrix }

	fn local_matrix(&self) -> Mat4 { self.local_matrix }
	fn set_local_matrix(&mut self, matrix: Mat4) { self.local_matrix = matrix }

	fn dirty(&self) -> bool { self.dirty }
	fn set_dirty(&mut self, dirty: bool) { self.dirty = dirty }
}

impl Default for Transform3D {
	fn default() -> Self {
		Self {
			owner: 0,
			scene: None,
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation: Vec3::ZERO,
			rotation: Quat::IDENTITY,
			scale: Vec3::ONE,
			dirty: true
		}
	}
}

impl Debug for Transform3D {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Transform3D")
			.field("owner", &self.owner)
			.field("local_matrix", &self.local_matrix)
			.field("global_matrix", &self.global_matrix)
			.field("translation", &self.translation)
			.field("rotation", &self.rotation)
			.field("scale", &self.scale)
			.field("dirty", &self.dirty)
			.finish()
	}
}

#[derive(Clone, NodeComponent)]
pub struct Transform2D {
	owner: NodeId,
	scene: Option<SharedSceneGraph>,

	pub(crate) local_matrix: Mat4,
	pub(crate) global_matrix: Mat4,

	translation: Vec2,
	rotation: f32,
	scale: Vec2,

	pub(crate) dirty: bool
}

impl Transform2D {
	pub fn new(translation: Vec2, rotation: f32, scale: Vec2) -> Self {
		Self {
			owner: 0,
			scene: None,
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
			owner: 0,
			scene: None,
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation,
			rotation: 0.0,
			scale: Vec2::ONE,
			dirty: true
		}
	}

	pub fn translate(&mut self, translation: Vec2) {
		self.translation += translation;
		self.dirty = true;
	}

	pub fn rotate(&mut self, rotation: f32) {
		self.rotation += rotation;
		self.dirty = true;
	}

	// pub fn scale(&mut self, scale: Vec3) {
	// 	self.scale += scale;
	// }

	pub fn translation(&self) -> Vec2 { self.translation }
	pub fn set_translation(&mut self, translation: Vec2) { self.translation = translation; self.dirty = true; }

	pub fn rotation(&self) -> f32 { self.rotation }
	pub fn set_rotation(&mut self, rotation: f32) { self.rotation = rotation; self.dirty = true; }

	pub fn scale(&self) -> Vec2 { self.scale }
	pub fn set_scale(&mut self, scale: Vec2) { self.scale = scale; self.dirty = true; }
}

impl Transform for Transform2D {
	fn calculate_matrix(&self) -> Mat4 {
		let scale = Vec3::new(self.scale.x, self.scale.y, 1.0);
		let rotation = Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, self.rotation);
		let translation = Vec3::new(self.translation.x, self.translation.y, 0.0);

		let s = Mat4::from_scale(scale);
		let r = Mat4::from_quat(rotation);
		let t = Mat4::from_translation(translation);

		t * r * s
	}

	fn global_matrix(&self) -> Mat4 { self.global_matrix }
	fn set_global_matrix(&mut self, matrix: Mat4) { self.global_matrix = matrix }

	fn local_matrix(&self) -> Mat4 { self.local_matrix }
	fn set_local_matrix(&mut self, matrix: Mat4) { self.local_matrix = matrix }

	fn dirty(&self) -> bool { self.dirty }
	fn set_dirty(&mut self, dirty: bool) { self.dirty = dirty }
}

impl Default for Transform2D {
	fn default() -> Self {
		Self {
			owner: 0,
			scene: None,
			local_matrix: Mat4::IDENTITY,
			global_matrix: Mat4::IDENTITY,
			translation: Vec2::ZERO,
			rotation: 0.0,
			scale: Vec2::ONE,
			dirty: true
		}
	}
}

impl Debug for Transform2D {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Transform2D")
			.field("owner", &self.owner)
			.field("local_matrix", &self.local_matrix)
			.field("global_matrix", &self.global_matrix)
			.field("translation", &self.translation)
			.field("rotation", &self.rotation)
			.field("scale", &self.scale)
			.field("dirty", &self.dirty)
			.finish()
	}
}
