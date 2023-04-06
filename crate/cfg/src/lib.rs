//! Configuration model for the peace automation library.
//!
//! This crate defines the API for logic and data to be used in the `peace`
//! framework.

// Re-exports
pub use async_trait::async_trait;
#[cfg(feature = "output_progress")]
pub use peace_core::progress;

pub use peace_core::*;

pub use crate::{item_spec::ItemSpec, op_ctx::OpCtx, state::State};

pub mod accessors;
pub mod state;

mod item_spec;
mod op_ctx;
