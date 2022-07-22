//! Runtime resources for the peace automation framework.
//!
//! This crate contains resources necessary for the peace framework to work, and
//! are likely to be common use for all applications.

// Re-exports
pub use type_reg;

pub use crate::{
    full_spec_rt_id::FullSpecRtId, full_spec_states::FullSpecStates,
    full_spec_states_desired::FullSpecStatesDesired,
    full_spec_states_desired_rw::FullSpecStatesDesiredRw, full_spec_states_rw::FullSpecStatesRw,
    resources::Resources,
};

pub mod resources_type_state;

mod full_spec_rt_id;
mod full_spec_states;
mod full_spec_states_desired;
mod full_spec_states_desired_rw;
mod full_spec_states_rw;
mod resources;
