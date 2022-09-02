//! Runtime resources for the peace automation framework.
//!
//! This crate contains resources necessary for the peace framework to work, and
//! are likely to be common use for all applications.

// Re-exports
pub use type_reg;

pub use crate::{item_spec_rt_id::ItemSpecRtId, resources::Resources};

pub mod internal;
pub mod paths;
pub mod resources_type_state;
pub mod states;

mod item_spec_rt_id;
mod resources;
