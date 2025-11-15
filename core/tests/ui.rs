use std::{path::{Path, PathBuf}, str::FromStr};

use fatum::{Application, ApplicationInfo, CoreEngine, OutputKind, components::UiElement, resources::{ResText, ResTexture2D}};
use fatum_graphics::{platform::{GraphicsPlatform, opengl::OpenGlPlatform}, render::PipelineKind};
use fatum_resources::ResourcePlatform;
use fatum_scene::{Node, SceneGraph};
use winit::{event_loop::EventLoop, platform::x11::EventLoopBuilderExtX11};

struct GuiApplication<P: GraphicsPlatform + ResourcePlatform> {
	_marker: std::marker::PhantomData<P>
}

impl<P: GraphicsPlatform + ResourcePlatform + Clone> Application<P> for GuiApplication<P> {
	fn info() -> ApplicationInfo {
		ApplicationInfo {
			name: String::from("GUI Test")
		}
	}

	fn setup(&mut self, engine: &mut CoreEngine<P, Self>, event_loop: &EventLoop<()>) where Self: Sized {
		engine.graphics_engine().create_queue(0, PipelineKind::PBR);
		engine.graphics_engine().create_output(0, event_loop, OutputKind::Window);
		
		let scene = SceneGraph::new();

		{
			let mut scene = scene.write().unwrap();

			let mut element = Node::new();
			element.add_component(Box::new(UiElement::new(|delta, element, ctx| {
				egui::Window::new("Hello").show(ctx, |ui| {
					ui.heading("Meow!");

					if ui.button("Awoo?").clicked() {
						log::info!("Awoooooooooooo");
					}
				});
			})));

			scene.add_node(element, None);
		}

		engine.scene_engine().set_scene(0, scene);
	}
}

impl<P: GraphicsPlatform + ResourcePlatform> Default for GuiApplication<P> {
	fn default() -> Self {
		Self {
			_marker: Default::default()
		}
	}
}

#[test]
fn gui_application() {
	fatum::build::link_test_assets();

	let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();

	let app = Box::new(GuiApplication::<OpenGlPlatform>::default());
	let mut engine = CoreEngine::<OpenGlPlatform, GuiApplication::<OpenGlPlatform>>::new(app, &event_loop);

	engine.setup(&event_loop);
	event_loop.run_app(&mut engine).unwrap();
}
