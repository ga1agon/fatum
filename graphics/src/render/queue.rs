use std::{cell::RefCell, rc::Rc};

use glam::Mat4;

use crate::{Camera, Model, Rf, render::{RenderObject, pipeline::RenderPipeline, target::RenderTarget}};

pub trait RenderQueue {
	fn begin_single(&mut self, index: usize) -> bool;
	fn end_single(&mut self, index: usize) -> bool;
	fn process_single(&mut self, index: usize) -> bool;

	fn process(&mut self);

	fn is_active(&self) -> bool;

	fn pipeline(&self) -> Option<&Box<dyn RenderPipeline>>;
	fn pipeline_mut(&mut self) -> Option<&mut Box<dyn RenderPipeline>>;
	fn set_pipeline(&mut self, pipeline: Option<Box<dyn RenderPipeline>>);

	fn targets(&self) -> Vec<usize>;
	fn add_target(&mut self, target: Box<dyn RenderTarget>) -> usize;
	fn get_target(&self, index: usize) -> Option<&Box<dyn RenderTarget>>;
	fn get_target_mut(&mut self, index: usize) -> Option<&mut Box<dyn RenderTarget>>;
	fn remove_target(&mut self, index: usize) -> bool;

	fn add_command(&mut self, command: fn(std::time::Duration)) -> usize;
	fn remove_command(&mut self, index: usize) -> bool;

	fn add_object(&mut self, object: &RenderObject, matrix: Mat4) -> bool;
	fn set_object_matrix(&mut self, object: &RenderObject, matrix: Mat4) -> bool;
	fn remove_object(&mut self, object: &RenderObject) -> bool;
	fn clear_objects(&mut self);
}
