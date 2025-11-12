use glam::{Mat4, Quat, Vec3, Vec4};

pub fn mat4_decompose(matrix: Mat4) -> (Vec3, Quat, Vec3) {
	let translation = matrix.w_axis.truncate();

	let x_axis = matrix.x_axis.truncate();
	let y_axis = matrix.y_axis.truncate();
	let z_axis = matrix.z_axis.truncate();

	let scale = Vec3::new(
		x_axis.length(), 
		y_axis.length(), 
		z_axis.length()
	);

	let inv_scale = Vec3::new(
		if scale.x != 0.0 { 1.0 / scale.x } else { 0.0 },
		if scale.y != 0.0 { 1.0 / scale.y } else { 0.0 },
		if scale.z != 0.0 { 1.0 / scale.z } else { 0.0 },
	);

	let rotation_matrix = Mat4::from_cols(
		matrix.x_axis * inv_scale.x,
		matrix.y_axis * inv_scale.y,
		matrix.z_axis * inv_scale.z,
		Vec4::W,
	);

	let rotation = Quat::from_mat4(&rotation_matrix);

	(translation, rotation, scale)
}
