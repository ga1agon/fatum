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
