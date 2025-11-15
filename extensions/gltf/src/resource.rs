use std::path::{Path, PathBuf};
use std::io::Write;
use std::rc::Rc;

use fatum::components;
use fatum::{components::{Model, Transform3D}, deserialize_metadata, serialize_metadata, write_resource_file};
use fatum_graphics::{Color, Vertex, texture};
use fatum_graphics::texture::Texture2D;
use fatum_graphics::{Material, platform::GraphicsPlatform};
use fatum_resources::{Resource, ResourceMetadata, ResourcePlatform, error::{ErrorKind, ResourceError}};
use fatum_scene::{NodeTree, NodeTreeEntry};
use glam::{Mat4, Vec2, Vec3, Vec3A};
use gltf::texture::{MagFilter, WrappingMode};
use gltf::{Gltf, Semantic};
use image::{DynamicImage, Rgb, Rgba};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaGltfScene {
	pub id: u64,
	pub format: String
}

impl ResourceMetadata for MetaGltfScene {
	fn default() -> Self where Self: Sized {
		Self {
			id: fatum_resources::next_id(),
			format: String::from("gltf_scene")
		}
	}

	fn id(&self) -> u64 { self.id }
	fn format(&self) -> &str { &self.format }
}

pub struct ResGltfScene {
	path: PathBuf,
	metadata: MetaGltfScene,
	value: NodeTree
}

impl ResGltfScene {
	pub fn get(&self) -> &NodeTree { &self.value }
}

impl<P: GraphicsPlatform + ResourcePlatform + Sized> Resource<P> for ResGltfScene {
	fn load(manager: &fatum_resources::Resources<P>, path: PathBuf, metadata: Option<std::fs::File>, asset: std::fs::File) -> Result<Self, fatum_resources::error::ResourceError>
		where Self: Sized
	{
		let metadata = deserialize_metadata!(metadata, path, MetaGltfScene::default());

		let gltf = Gltf::from_reader(asset)
			.map_err(|e| ResourceError::new(&path, ErrorKind::LoadError, format!("Couldn't load glTF file: {}", e).as_str()))?;

		let buffer_data = gltf::import_buffers(
			&gltf.document,
			Some(path.parent().unwrap_or_else(|| Path::new("./"))), 
			gltf.blob.clone()
		).map_err(|e| ResourceError::new(&path, ErrorKind::LoadError, format!("Couldn't import glTF document buffer data: {}", e).as_str()))?;

		let mut image_data = gltf::import_images(
			&gltf.document,
			Some(path.parent().unwrap_or_else(|| Path::new("./"))),
			&buffer_data
		).map_err(|e| ResourceError::new(&path, ErrorKind::LoadError, format!("Couldn't import glTF document image data: {}", e).as_str()))?;

		let mut tree = NodeTree::new();
		tree.root.components.push(Box::new(Transform3D::default()));

		fn process_node<P: GraphicsPlatform + ResourcePlatform + Sized>(
			platform: Rc<P>,
			buffer_data: &Vec<gltf::buffer::Data>,
			image_data: &mut Vec<gltf::image::Data>,
			scene_node: &mut NodeTreeEntry,
			node: gltf::Node<'_>
		) {
			let mut tree_node = NodeTreeEntry::new();

			let t3d = Transform3D::from_mat4(Mat4::from_cols_array_2d(&node.transform().matrix()));
			tree_node.components.push(Box::new(t3d));
			
			if let Some(mesh) = node.mesh() {
				let mut meshes = Vec::new();

				for primitive in mesh.primitives() {
					let mut mesh = fatum_graphics::Mesh {
						vertices: Vec::new(),
						indices: Vec::new(),
						material: Material::default()
					};

					let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

					// vertex attributes
					if let Some(iter) = reader.read_positions() {
						for position in iter {
							let v = Vertex::new(Vec3::from_array(position), Default::default(), Default::default(), Default::default(), Default::default());
							mesh.vertices.push(v);
						}
					}

					if let Some(iter) = reader.read_normals() {
						for (i, normal) in iter.enumerate() {
							if let Some(v) = mesh.vertices.get_mut(i) {
								v.normal = Vec3A::from_array(normal);
							}
						}
					}

					if let Some(iter) = reader.read_tangents() {
						for (i, tangent) in iter.enumerate() {
							if let Some(v) = mesh.vertices.get_mut(i) {
								v.tangent = Vec3A::new(tangent[0], tangent[1], tangent[2]);
								v.bitangent = v.normal * v.tangent;
							}
						}
					}

					if let Some(iter) = reader.read_tex_coords(0) {
						for (i, uv) in iter.into_f32().enumerate() {
							if let Some(v) = mesh.vertices.get_mut(i) {
								v.uv = Vec2::from_array(uv);
							}
						}
					}

					// indices
					if let Some(iter) = reader.read_indices() {
						for index in iter.into_u32() {
							mesh.indices.push(index);
						}
					}

					// material
					let material = primitive.material();

					let base_color = material.pbr_metallic_roughness().base_color_factor();
					let metalness = material.pbr_metallic_roughness().metallic_factor();
					let roughness = material.pbr_metallic_roughness().roughness_factor();
					let ior = material.ior().unwrap_or(1.5);

					// textures
					let mut make_texture = |name: &str, texture: gltf::texture::Texture<'_>| -> Option<Box<dyn Texture2D>> {
						let image = texture.source();
						let data = image_data.get_mut(image.index()).unwrap();
						
						let pixels = std::mem::take(&mut data.pixels);
						let dyn_image: DynamicImage;

						// afaik there's no way to check what format the texture is, so we have to resort to this fuckery
						{
							let image = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(data.width, data.height, pixels.clone());

							if image.is_none() {
								let image = image::ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(data.width, data.height, pixels);

								if image.is_none() {
									log::error!("Couldn't load texture {}", name);
									return None;
								} else {
									log::debug!("Loaded texture {} as RGBA8", name);
									dyn_image = image::DynamicImage::ImageRgba8(image.unwrap());
								}
							} else {
								log::debug!("Loaded texture {} as RGB8", name);
								dyn_image = image::DynamicImage::ImageRgb8(image.unwrap());
							}
						}
						
						let sampler = texture.sampler();

						let options = texture::Options {
							filter: match sampler.mag_filter().unwrap_or(MagFilter::Linear) {
								MagFilter::Linear => texture::Filter::Linear,
								MagFilter::Nearest => texture::Filter::Nearest
							},
							wrap_mode: match sampler.wrap_s() {
								WrappingMode::ClampToEdge => texture::WrapMode::ClampToEdge,
								WrappingMode::Repeat => texture::WrapMode::Repeat,
								WrappingMode::MirroredRepeat => texture::WrapMode::RepeatMirror
							},
							format: texture::Format::RGBA8,
							flip_v: false
						};

						if let Ok(texture) = platform.create_texture_2d(dyn_image, options) {
							return Some(texture);
						} else {
							log::error!("Couldn't create texture {}", name);
							return None;
						}
					};

					let mut base_map = None;
					let mut metalness_roughness_map = None;
					let mut normal_map = None;

					if let Some(texture) = material.pbr_metallic_roughness().base_color_texture() {
						base_map = make_texture("base", texture.texture());
					}

					// we will take one for both in the material and sample properly in the shader
					if let Some(texture) = material.pbr_metallic_roughness().metallic_roughness_texture() {
						metalness_roughness_map = make_texture("metalness-roughness", texture.texture());
					}

					if let Some(texture) = material.normal_texture() {
						normal_map = make_texture("normal", texture.texture());
					}

					let material = Material::with_textures_pbr(
						Color::from_rgba_f32(base_color[0], base_color[1], base_color[2], base_color[3]),
						metalness,
						roughness,
						ior,
						base_map.as_ref(),
						metalness_roughness_map.as_ref(),
						metalness_roughness_map.as_ref(),
						normal_map.as_ref(),
						None // glTF still has no displacement/height maps :/
					);

					mesh.material = material;
					meshes.push(mesh);
				}

				let model = Rc::new(Box::new(fatum_graphics::Model {
					meshes
				}));

				tree_node.components.push(Box::new(components::Model::new(model)));
			}

			scene_node.children.push(tree_node);

			for child in node.children() {
				process_node(platform.clone(), buffer_data, image_data, scene_node, child);
			}
		}

		for scene in gltf.scenes() {
			let mut scene_node = NodeTreeEntry::new();
			scene_node.components.push(Box::new(Transform3D::default()));
			
			for node in scene.nodes() {
				process_node(manager.platform.clone(), &buffer_data, &mut image_data, &mut scene_node, node);
			}

			tree.root.children.push(scene_node);
		}

		Ok(Self {
			path,
			metadata,
			value: tree
		})
	}

	fn save(&self, path: PathBuf, mut metadata: std::fs::File, asset: std::fs::File) -> Result<(), fatum_resources::error::ResourceError> {
		let metadata_value = serialize_metadata!(self.metadata, path)?;
		write_resource_file!(metadata, path, metadata_value.as_bytes())?;

		Ok(())
	}

	fn reload(&mut self) {
		todo!()
	}

	fn path(&self) -> &PathBuf { &self.path }
	fn metadata(&self) -> &dyn ResourceMetadata { &self.metadata }

	fn as_any(&self) -> &dyn std::any::Any { self }
}
