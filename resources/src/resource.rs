use std::{fs::File, sync::atomic::{AtomicU64, AtomicUsize}};

use crate::ResourceMetadata;

pub trait ResourcePlatform {}

const ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub trait Resource {
	fn load<M: ResourceMetadata + Sized>(metadata: M, file: &File) -> Self
		where Self: Sized;
	fn save(&self);
	fn reload(&mut self);

	fn metadata(&self) -> &dyn ResourceMetadata;
}
