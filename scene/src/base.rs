// use crate::{Node, lock_opt_mutex_unchecked};

// pub struct BaseNode {
// 	id: u32,
// 	name: String,
// 	scene: Option<std::sync::Arc<std::sync::Mutex<crate::SceneGraph>>>,
// }

// impl BaseNode {
// 	pub fn new(id: u32, name: &str) -> Self {
// 		Self {
// 			id,
// 			name: name.to_string(),
// 			scene: None
// 		}
// 	}
// }

// impl Node for BaseNode {
// 	fn id(&self) -> u32 {
// 		self.id
// 	}

// 	fn name(&self) -> &str {
// 		&self.name
// 	}

// 	fn set_name(&mut self, name: &str) {
// 		self.name = name.to_string()
// 	}

// 	fn scene(&self) -> Option<std::sync::Arc<std::sync::Mutex<crate::SceneGraph>>> {
// 		self.scene.clone()
// 	}

// 	fn parent(&self) -> u32 {
// 		lock_opt_mutex_unchecked(&self.scene).parent(self.id)
// 	}

// 	fn children(&self) -> Vec<u32> {
// 		lock_opt_mutex_unchecked(&self.scene).children(self.id)
// 	}

// 	fn enter_scene(&mut self, id: u32, scene: std::sync::Arc<std::sync::Mutex<crate::SceneGraph>>) {
// 		self.id = id;
// 		self.scene = Some(scene);
// 	}

// 	fn exit_scene(&mut self) {
// 		self.id = 0;
// 		self.scene = None;
// 	}
	
// 	fn as_any(&self) -> &dyn std::any::Any {
// 		self
// 	}
// }
