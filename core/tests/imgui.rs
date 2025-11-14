use std::{path::{Path, PathBuf}, str::FromStr};

use fatum::{Application, ApplicationInfo, CoreEngine, OutputKind, components::UiElement, resources::{ResText, ResTexture2D}};
use fatum_graphics::{Window, platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::PipelineKind};
use fatum_resources::ResourcePlatform;
use fatum_scene::{Node, SceneGraph};

struct ImGuiApplication<P: GraphicsPlatform + ResourcePlatform> {
	_marker: std::marker::PhantomData<P>
}

impl<P: GraphicsPlatform + ResourcePlatform + Clone> Application<P> for ImGuiApplication<P> {
	fn info() -> ApplicationInfo {
		ApplicationInfo {
			name: String::from("ImGui Test")
		}
	}

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>) where Self: Sized {
		engine.graphics_engine().create_output(0, PipelineKind::Default, OutputKind::Window);
		
		let scene = SceneGraph::new();

		{
			let mut scene = scene.write().unwrap();

			let mut element = Node::new();
			element.add_component(Box::new(UiElement::new(|delta, element, ui| {
				ui.window("Meow")
					.build(|| {
						ui.text("Meowowow meowww!");

						if ui.button("Awoo?") {
							log::info!("awoooooooooo");
						}
					});
			})));

			scene.add_node(element, None);
		}

		engine.scene_engine().set_scene(0, scene);
	}
}

impl<P: GraphicsPlatform + ResourcePlatform> Default for ImGuiApplication<P> {
	fn default() -> Self {
		Self {
			_marker: Default::default()
		}
	}
}

#[test]
fn imgui_application() {
	fatum::build::link_test_assets();

	let app = Box::new(ImGuiApplication::<OpenGlPlatform>::default());
	let mut engine = CoreEngine::<OpenGlPlatform, ImGuiApplication::<OpenGlPlatform>>::new(app);

	engine.setup();
	engine.run();
}
