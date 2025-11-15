pub mod error;

mod resource;

pub use resource::*;

mod manager;
pub use manager::*;

mod metadata;
pub use metadata::*;

pub const RESOURCE_ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

pub trait ResourcePlatform {}

type Rf<T> = std::rc::Rc<std::cell::RefCell<T>>;
pub(crate) fn rf<T>(v: T) -> Rf<T> {
	std::rc::Rc::new(std::cell::RefCell::new(v))
}

pub fn next_id() -> u64 {
	RESOURCE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}
