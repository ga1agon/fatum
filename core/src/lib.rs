#![feature(thread_local)]
#![feature(import_trait_associated_functions)]
pub mod resources;
pub mod build;
pub mod components;
pub mod nodes;
pub mod helpers;

mod app;
use std::rc::Rc;

pub use app::*;

mod engine;
pub use engine::*;

#[cfg(feature = "macros")]
pub use fatum_macros::*;
