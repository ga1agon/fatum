use fatum_graphics::{Model, Mesh, Vertex, Material};
use static_init::dynamic;
use glam::{Vec2, Vec3};
use std::default::Default::default as D;

#[dynamic]
pub static UNIT_CUBE: Model = Model {
	meshes: vec![
		Mesh {
			vertices: vec![
				Vertex::new(Vec3::new(-0.5,  0.5,  0.5), D(), D(), D(), Vec2::new(0.0, 1.0)),
				Vertex::new(Vec3::new(-0.5, -0.5,  0.5), D(), D(), D(), Vec2::new(0.0, 0.0)),
				Vertex::new(Vec3::new( 0.5, -0.5,  0.5), D(), D(), D(), Vec2::new(1.0, 0.0)),
				Vertex::new(Vec3::new( 0.5,  0.5,  0.5), D(), D(), D(), Vec2::new(1.0, 1.0)),
				Vertex::new(Vec3::new( 0.5,  0.5,  0.5), D(), D(), D(), Vec2::new(0.0, 1.0)),
				Vertex::new(Vec3::new( 0.5, -0.5,  0.5), D(), D(), D(), Vec2::new(0.0, 0.0)),
				Vertex::new(Vec3::new( 0.5, -0.5, -0.5), D(), D(), D(), Vec2::new(1.0, 0.0)),
				Vertex::new(Vec3::new( 0.5,  0.5, -0.5), D(), D(), D(), Vec2::new(1.0, 1.0)),
				Vertex::new(Vec3::new( 0.5,  0.5, -0.5), D(), D(), D(), Vec2::new(0.0, 1.0)),
				Vertex::new(Vec3::new( 0.5, -0.5, -0.5), D(), D(), D(), Vec2::new(0.0, 0.0)),
				Vertex::new(Vec3::new(-0.5, -0.5, -0.5), D(), D(), D(), Vec2::new(1.0, 0.0)),
				Vertex::new(Vec3::new(-0.5,  0.5, -0.5), D(), D(), D(), Vec2::new(1.0, 1.0)),
				Vertex::new(Vec3::new(-0.5,  0.5, -0.5), D(), D(), D(), Vec2::new(0.0, 1.0)),
				Vertex::new(Vec3::new(-0.5, -0.5, -0.5), D(), D(), D(), Vec2::new(0.0, 0.0)),
				Vertex::new(Vec3::new(-0.5, -0.5,  0.5), D(), D(), D(), Vec2::new(1.0, 0.0)),
				Vertex::new(Vec3::new(-0.5,  0.5,  0.5), D(), D(), D(), Vec2::new(1.0, 1.0)),
				Vertex::new(Vec3::new(-0.5,  0.5, -0.5), D(), D(), D(), Vec2::new(0.0, 0.0)),
				Vertex::new(Vec3::new(-0.5,  0.5,  0.5), D(), D(), D(), Vec2::new(0.0, 1.0)),
				Vertex::new(Vec3::new( 0.5,  0.5,  0.5), D(), D(), D(), Vec2::new(1.0, 1.0)),
				Vertex::new(Vec3::new( 0.5,  0.5, -0.5), D(), D(), D(), Vec2::new(1.0, 0.0)),
				Vertex::new(Vec3::new(-0.5, -0.5,  0.5), D(), D(), D(), Vec2::new(0.0, 0.0)),
				Vertex::new(Vec3::new(-0.5, -0.5, -0.5), D(), D(), D(), Vec2::new(0.0, 1.0)),
				Vertex::new(Vec3::new( 0.5, -0.5, -0.5), D(), D(), D(), Vec2::new(1.0, 1.0)),
				Vertex::new(Vec3::new( 0.5, -0.5,  0.5), D(), D(), D(), Vec2::new(1.0, 0.0))
			],
			indices: vec![
				0, 1, 2, 0, 2, 3,
				4, 5, 6, 4, 6, 7,
				8, 9, 10, 8, 10, 11,
				12, 13, 14, 12, 14, 15,
				16, 17, 18, 16, 18, 19,
				20, 21, 22, 20, 22, 23
			],
			material: Material::default()
		}
	]
};
