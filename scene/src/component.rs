use std::sync::{Arc, Mutex};

use crate::{NodeId, SceneGraph, SharedSceneGraph};

pub trait NodeComponent: 'static {
	fn name(&self) -> &str;

	fn enter_scene(&mut self, owner: NodeId, scene: SharedSceneGraph);
	fn exit_scene(&mut self);

	fn clone_component(&self) -> Box<dyn NodeComponent>;

	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
