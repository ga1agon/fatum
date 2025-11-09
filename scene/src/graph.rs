use serde::{Deserialize, Serialize};

use crate::{Node, SceneTree};

#[derive(Serialize, Deserialize)]
pub struct NodeGraph {
	
}

impl NodeGraph {
	pub fn instantiate_to_node(&self) -> Node {
		todo!()
	}

	pub fn instantiate_to_scene(&self) -> SceneTree {
		todo!()
	}
}
