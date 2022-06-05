//! Configuration model for the peace automation library.
//!
//! This crate defines the API for logic and data to be used in the `peace`
//! framework.

// Re-exports
pub use async_trait::async_trait;
pub use diff;

pub use crate::{
    fn_spec::FnSpec, full_spec::FullSpec, op_check_status::OpCheckStatus, op_spec::OpSpec,
    op_spec_dry::OpSpecDry, progress_limit::ProgressLimit,
};

mod fn_spec;
mod full_spec;
mod op_check_status;
mod op_spec;
mod op_spec_dry;
mod progress_limit;
