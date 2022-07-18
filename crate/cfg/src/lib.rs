//! Configuration model for the peace automation library.
//!
//! This crate defines the API for logic and data to be used in the `peace`
//! framework.

// Re-exports
pub use async_trait::async_trait;
pub use nougat;
pub use peace_core::{FullSpecId, FullSpecIdInvalidFmt};
pub use peace_full_spec_id_macro::full_spec_id;

#[nougat::gat(Data)]
pub use crate::clean_op_spec::CleanOpSpec;
#[nougat::gat(Data)]
pub use crate::ensure_op_spec::EnsureOpSpec;
#[nougat::gat(Data)]
pub use crate::fn_spec::FnSpec;
pub use crate::{
    full_spec::FullSpec, op_check_status::OpCheckStatus, progress_limit::ProgressLimit,
    state::State,
};

mod clean_op_spec;
mod ensure_op_spec;
mod fn_spec;
mod full_spec;
mod op_check_status;
mod progress_limit;
mod state;
