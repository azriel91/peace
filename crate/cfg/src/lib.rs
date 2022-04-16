//! Configuration model for the zzzz automation library.
//!
//! This crate defines the API for logic and data to be used in the `zzzz`
//! framework.

pub use crate::{op_spec::OpSpec, work_spec::WorkSpec};

mod op_spec;
mod work_spec;
