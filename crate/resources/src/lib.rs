//! Runtime resources for the peace automation framework.
//!
//! This crate contains resources necessary for the peace framework to work, and
//! are likely to be common use for all applications.

// Re-exports
pub use rt_vec;

pub use crate::{
    full_spec_resourceses::FullSpecResourceses, full_spec_rt_id::FullSpecRtId, resources::Resources,
};

pub mod resources_type_state;

mod full_spec_resourceses;
mod full_spec_rt_id;
mod resources;
