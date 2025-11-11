use std::sync::{Arc, Mutex};

use crate::{NodeId, SceneGraph, SharedSceneGraph};

pub trait NodeComponent: 'static {
	fn enter_scene(&mut self, owner: NodeId, scene: SharedSceneGraph);
	fn exit_scene(&mut self);

	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

// impl<T: 'static> NodeComponent for T {
// 	fn as_any(&self) -> &dyn std::any::Any { self }
// 	fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
// }
