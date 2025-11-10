mod scene;
pub use scene::*;

mod node;
pub use node::*;

mod tree;
pub use tree::*;

mod component;
pub use component::*;

mod behaviour;
pub use behaviour::*;

mod base;
pub use base::*;

#[cfg(feature = "macros")]
pub use fatum_scene_macros::*;

pub(crate) fn lock_opt_mutex_unchecked<T>(opt_mutex: &Option<std::sync::Arc<std::sync::Mutex<T>>>) -> std::sync::MutexGuard<'_, T> {
	opt_mutex.as_ref().unwrap().lock().unwrap()
}
