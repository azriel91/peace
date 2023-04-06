//! Configuration model for the peace automation library.
//!
//! This crate defines the API for logic and data to be used in the `peace`
//! framework.

// Re-exports
pub use async_trait::async_trait;
#[cfg(feature = "output_progress")]
pub use peace_core::progress;

pub use peace_core::*;

pub use crate::{
    apply_op_spec::ApplyOpSpec, item_spec::ItemSpec, op_ctx::OpCtx, state::State,
    state_diff_fn_spec::StateDiffFnSpec,
};

pub mod accessors;
pub mod state;

mod apply_op_spec;
mod item_spec;
mod op_ctx;
mod state_diff_fn_spec;
