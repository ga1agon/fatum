mod opengl;
use dear_imgui_rs::Ui;
use opengl::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::task::Context;
use fatum_graphics::platform::{GraphicsContext, GraphicsPlatform};
use fatum_graphics::platform::opengl::OpenGlPlatform;
use fatum_scene::iterators::{SceneDfsIterator, ScenePostDfsIterator};
use crate::{GraphicsEngine, SceneEngine};
use crate::components::UiElement;

pub struct UiEngine<P: GraphicsPlatform> {
	graphics: Rc<RefCell<GraphicsEngine<P>>>,
	scene: Rc<RefCell<SceneEngine<P>>>,
	context: dear_imgui_rs::Context,
	glow_renderer: Option<dear_imgui_glow::GlowRenderer>
}

impl<P> UiEngine<P> where P: GraphicsPlatform {
	pub fn new(graphics: Rc<RefCell<GraphicsEngine<P>>>, scene: Rc<RefCell<SceneEngine<P>>>) -> Self {
		let mut context: dear_imgui_rs::Context;
		let mut glow_renderer: Option<dear_imgui_glow::GlowRenderer> = None;

		{
			let mut graphics = graphics.borrow_mut();
			let graphics = graphics.get();

			if let Some(ogl_platform) = graphics.as_any().downcast_ref::<OpenGlPlatform>() {
				let glow_context = ogl_platform.context().get();
				context = dear_imgui_rs::Context::create();

				let texture_map = Box::new(OglImGuiTextureMap::new());
				let renderer = dear_imgui_glow::GlowRenderer::with_external_context(glow_context.as_ref(), &mut context, texture_map)
					.expect("Failed to create an ImGui context for the OpenGL platform");

				glow_renderer = Some(renderer);

				log::info!("Created ImGui context for OpenGL");
			} else {
				todo!()
			}
		}

		log::info!("Created UI engine");

		Self {
			graphics,
			scene,
			context,
			glow_renderer
		}
	}

	pub fn process(&mut self, delta: std::time::Duration) -> bool {
		let scene_engine = self.scene.borrow();

		let imgui = &mut self.context;
		let frame = imgui.frame();

		for (output, scene) in scene_engine.scenes() {
			if let Some(queue) = self.graphics.borrow_mut().get_output(*output) {
				let nodes: Vec<u32> = SceneDfsIterator::new(scene.clone(), Default::default())
					.collect();

				if let Ok(scene) = scene.try_read() {
					for node in &nodes {
						let node = scene.node(*node)
							.expect("Iterator returned a non-existing node");

						if let Some(element) = node.component::<UiElement>() {
							element.draw(delta, frame);
						}
					}
				} else {
					log::warn!("Could not get a write lock on scene {}; UI nodes will not be processed", *output);
				}
			}
		}

		let draw_data = imgui.render();

		if let Some(renderer) = &mut self.glow_renderer {
			if let Err(e) = renderer.new_frame() {
				log::warn!("Could not create a new frame with the Glow renderer: {}", e);
			} else {
				_ = renderer.render(draw_data)
					.inspect_err(|e| log::warn!("Could not render draw data with the Glow renderer: {}", e));
			}
		} else {
			log::warn!("No renderer instance available to render ImGui with");
		}

		true
	}
}
