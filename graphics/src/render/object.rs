use std::{hash::Hash, sync::atomic::{AtomicU64, Ordering}};

use glam::Mat4;

use crate::Model;

#[derive(Debug)]
pub struct RenderObject {
	pub id: u64,
	pub model: Model
}

impl RenderObject {
	const ID_COUNTER: AtomicU64 = AtomicU64::new(1);

	pub fn new(model: Model) -> Self {
		Self::with_id(Self::ID_COUNTER.fetch_add(1, Ordering::Relaxed), model)
	}

	pub fn with_id(id: u64, model: Model) -> Self {
		Self {
			id,
			model
		}
	}
}

impl PartialEq for RenderObject {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for RenderObject {}

impl Hash for RenderObject {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}
