//! Configuration model for the peace automation library.
//!
//! This crate defines the API for logic and data to be used in the `peace`
//! framework.

// Re-exports
pub use async_trait::async_trait;
#[cfg(feature = "output_progress")]
pub use peace_core::progress;

pub use peace_core::*;

pub use crate::{fn_ctx::FnCtx, item_spec::ItemSpec, state::State};

pub mod accessors;
pub mod state;

mod fn_ctx;
mod item_spec;
