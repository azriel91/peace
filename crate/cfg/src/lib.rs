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
    clean_op_spec::CleanOpSpec, ensure_op_spec::EnsureOpSpec, item_spec::ItemSpec, op_ctx::OpCtx,
    saved::Saved, state::State, state_diff_fn_spec::StateDiffFnSpec, try_fn_spec::TryFnSpec,
};

pub mod state;

mod clean_op_spec;
mod ensure_op_spec;
mod item_spec;
mod op_ctx;
mod saved;
mod state_diff_fn_spec;
mod try_fn_spec;
