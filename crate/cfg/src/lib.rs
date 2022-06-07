//! Configuration model for the peace automation library.
//!
//! This crate defines the API for logic and data to be used in the `peace`
//! framework.

// Re-exports
pub use async_trait::async_trait;
pub use diff;

pub use crate::{
    clean_op_spec::CleanOpSpec, ensure_op_spec::EnsureOpSpec, fn_spec::FnSpec, full_spec::FullSpec,
    op_check_status::OpCheckStatus, progress_limit::ProgressLimit,
};

mod clean_op_spec;
mod ensure_op_spec;
mod fn_spec;
mod full_spec;
mod op_check_status;
mod progress_limit;
