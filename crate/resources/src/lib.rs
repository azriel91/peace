//! Runtime resources for the peace automation framework.
//!
//! This crate contains resources necessary for the peace framework to work, and
//! are likely to be common use for all applications.

// Re-exports
pub use type_reg;

pub use crate::{
    full_spec_rt_id::FullSpecRtId, resources::Resources, states::States,
    states_desired::StatesDesired, states_desired_rw::StatesDesiredRw, states_rw::StatesRw,
};

pub mod resources_type_state;

mod full_spec_rt_id;
mod resources;
mod states;
mod states_desired;
mod states_desired_rw;
mod states_rw;
