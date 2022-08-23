//! Low level data types for the peace automation framework.

// Re-exports
pub use peace_static_check_macros::{item_spec_id, profile};

pub use crate::{
    item_spec_id::ItemSpecId, item_spec_id_invalid_fmt::ItemSpecIdInvalidFmt,
    op_check_status::OpCheckStatus, profile::Profile, profile_invalid_fmt::ProfileInvalidFmt,
    progress_limit::ProgressLimit,
};

mod item_spec_id;
mod item_spec_id_invalid_fmt;
mod op_check_status;
mod profile;
mod profile_invalid_fmt;
mod progress_limit;
