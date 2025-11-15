use std::{any::Any, path::{Path, PathBuf}, rc::Rc, str::FromStr};

use fatum::{Application, ApplicationInfo, CoreEngine, OutputKind, components::{Transform2D, Transform3D}, nodes::{Camera3D, Model3D, UiElement, UiWindow}, resources::{ResText, ResTexture2D}};
use fatum_ext_gltf::ResGltfScene;
use fatum_graphics::{Color, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::PipelineKind};
use fatum_resources::ResourcePlatform;
use fatum_scene::{Node, NodeId, SceneGraph, iterators::SceneDfsIterator};
use glam::{EulerRot, UVec2, Vec2, Vec3};
use winit::{event_loop::EventLoop, platform::x11::EventLoopBuilderExtX11};

struct GltfViewerApplication<P: GraphicsPlatform + ResourcePlatform> {
	_marker: std::marker::PhantomData<P>
}

impl<P: GraphicsPlatform + ResourcePlatform + Clone> Application<P> for GltfViewerApplication<P> {
	fn info() -> ApplicationInfo {
		ApplicationInfo {
			name: String::from("glTF Viewer")
		}
	}

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>, event_loop: &EventLoop<()>) where Self: Sized {
		engine.graphics_engine().create_queue(0, PipelineKind::Default);
		engine.graphics_engine().create_output(0, event_loop, OutputKind::Window);

		let gltf1 = engine.resource_engine().get().load_by_path::<ResGltfScene>("modern_arm_chair_01/modern_arm_chair_01_1k.gltf", false).unwrap();
		let gltf2 = engine.resource_engine().get().load_by_path::<ResGltfScene>("mid_century_lounge_chair/mid_century_lounge_chair_1k.gltf", false).unwrap();

		let scene = SceneGraph::new();
		let gltf1 = gltf1.borrow().get().instantiate(scene.clone(), None);
		let gltf2 = gltf2.borrow().get().instantiate(scene.clone(), None);

		{
			let mut scene = scene.write().unwrap();

			{
				let gltf1 = scene.node_mut(gltf1).unwrap();
				gltf1.set_name("Modern arm chair");

				let t3d = gltf1.component_mut::<Transform3D>().unwrap();
				t3d.translate(Vec3::new(-2.0, 0.0, 0.0));
				t3d.rotate_euler(EulerRot::XYZ, Vec3::new(0.0, 180.0f32.to_radians(), 0.0));
				t3d.set_scale(Vec3::new(2.0, 2.0, 2.0));
			}

			{
				let gltf2 = scene.node_mut(gltf2).unwrap();
				gltf2.set_name("Mid-century lounge chair");

				let t3d = gltf2.component_mut::<Transform3D>().unwrap();
				t3d.translate(Vec3::new(2.0, 0.0, 0.0));
				t3d.rotate_euler(EulerRot::XYZ, Vec3::new(0.0, 180.0f32.to_radians(), 0.0));
				t3d.set_scale(Vec3::new(2.0, 2.0, 2.0));
			}
		}

		{
			{
				let mut scene = scene.write().unwrap();

				{
					let mut node = Camera3D::new_perspective(UVec2::new(1024, 768), 60.0, true);
					node.component_mut::<Transform3D>().unwrap()
						.translate(Vec3::new(-5.0, 4.0, -5.0));
					node.component_mut::<Transform3D>().unwrap()
						.rotate_euler(EulerRot::YXZ, Vec3::new(45.0f32.to_radians(), 25.0f32.to_radians(), 0.0));

					scene.add_node(node.into(), None);
				}
			}

			{
				let scene1 = scene.clone();

				let mut node = UiWindow::new(String::from("Scene"), move |_, _, ui| {
					let nodes: Vec<u32> = SceneDfsIterator::new(scene1.clone(), 0).collect();
					let scene = scene1.read().unwrap();

					for id in nodes {
						let node = scene.node(id).unwrap();
						let components = node.components();

						ui.label(format!("> {} - {}", id, node.name()));

						for component in components {
							ui.label(component.name());
						}
					}
				});

				{
					let mut scene = scene.write().unwrap();
					scene.add_node(node, None);
				}
			}
		}

		engine.scene_engine().set_scene(0, scene);
	}
}

impl<P: GraphicsPlatform + ResourcePlatform> Default for GltfViewerApplication<P> {
	fn default() -> Self {
		Self {
			_marker: Default::default()
		}
	}
}

#[test]
fn gltf_viewer() {
	fatum::build::link_assets(
		"../extensions/gltf/tests/assets",
		PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap().join("../../target/debug/deps").to_str()
	);

	let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();

	let app = Box::new(GltfViewerApplication::<OpenGlPlatform>::default());
	let mut engine = CoreEngine::<OpenGlPlatform, GltfViewerApplication::<OpenGlPlatform>>::new(app, &event_loop);

	engine.setup(&event_loop);
	event_loop.run_app(&mut engine).unwrap();
}
