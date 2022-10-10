//! Configuration model for the peace automation library.
//!
//! This crate defines the API for logic and data to be used in the `peace`
//! framework.

// Re-exports
pub use async_trait::async_trait;
pub use nougat;
pub use peace_core::{
    flow_id, item_spec_id, profile, FlowId, FlowIdInvalidFmt, ItemSpecId, ItemSpecIdInvalidFmt,
    OpCheckStatus, Profile, ProfileInvalidFmt, ProgressLimit,
};

#[nougat::gat(Data)]
pub use crate::clean_op_spec::CleanOpSpec;
#[nougat::gat(Data)]
pub use crate::ensure_op_spec::EnsureOpSpec;
#[nougat::gat(Data)]
pub use crate::fn_spec::FnSpec;
#[nougat::gat(Data)]
pub use crate::state_diff_fn_spec::StateDiffFnSpec;
pub use crate::{item_spec::ItemSpec, state::State};

pub mod state;

mod clean_op_spec;
mod ensure_op_spec;
mod fn_spec;
mod item_spec;
mod state_diff_fn_spec;
