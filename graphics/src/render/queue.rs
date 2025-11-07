use std::rc::Rc;

use crate::{Model, render::{RenderObject, pipeline::RenderPipeline, target::RenderTarget}};

pub trait RenderQueue {
	fn process(&mut self);

	fn pipeline(&self) -> &Option<Box<dyn RenderPipeline>>;
	fn set_pipeline(&mut self, pipeline: Option<Box<dyn RenderPipeline>>);

	fn add_target(&mut self, target: Box<dyn RenderTarget>);
	fn add_command(&mut self, command: fn(std::time::Duration));

	fn add_object(&mut self, object: Rc<RenderObject>) -> bool;
	fn remove_object(&mut self, object: &RenderObject) -> bool;
}
