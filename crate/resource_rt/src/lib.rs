//! Runtime resources for the peace automation framework.
//!
//! This crate contains resources necessary for the peace framework to work, and
//! are likely to be common use for all applications.

// Re-exports
pub use resman::*;
pub use type_reg;

pub use crate::{item_rt_id::ItemRtId, resources::Resources};

pub mod internal;
pub mod paths;
pub mod resources;
pub mod states;

mod item_rt_id;
