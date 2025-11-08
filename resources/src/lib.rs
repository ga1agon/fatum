mod resource;
pub use resource::*;

mod manager;
pub use manager::*;

mod metadata;
pub use metadata::*;

type Rf<T> = std::rc::Rc<std::cell::RefCell<T>>;
pub(crate) fn rf<T>(v: T) -> Rf<T> {
	std::rc::Rc::new(std::cell::RefCell::new(v))
}
