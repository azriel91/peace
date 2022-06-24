//! Configuration model for the peace automation library.
//!
//! This crate defines the API for logic and data to be used in the `peace`
//! framework.

// Re-exports
pub use async_trait::async_trait;
pub use peace_full_spec_id_macro::full_spec_id;

pub use crate::{
    clean_op_spec::CleanOpSpec, ensure_op_spec::EnsureOpSpec, fn_spec::FnSpec, full_spec::FullSpec,
    full_spec_id::FullSpecId, full_spec_id_invalid_fmt::FullSpecIdInvalidFmt,
    op_check_status::OpCheckStatus, progress_limit::ProgressLimit, state::State,
};

mod clean_op_spec;
mod ensure_op_spec;
mod fn_spec;
mod full_spec;
mod full_spec_id;
mod full_spec_id_invalid_fmt;
mod op_check_status;
mod progress_limit;
mod state;
