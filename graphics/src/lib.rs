pub mod error;
pub mod platform;
pub mod render;
pub mod shader;
pub mod texture;

mod window;
pub use window::*;

mod vertex;
pub use vertex::*;

mod material;
pub use material::*;

mod color;
pub use color::*;

mod mesh;
pub use mesh::*;

mod model;
pub use model::*;

mod camera;
pub use camera::*;

type Rf<T> = std::rc::Rc<std::cell::RefCell<T>>;
pub(crate) fn rf<T>(v: T) -> Rf<T> {
	std::rc::Rc::new(std::cell::RefCell::new(v))
}
