//! Runtime resources for the peace automation framework.
//!
//! This crate contains resources necessary for the peace framework to work, and
//! are likely to be common use for all applications.

// Re-exports
pub use type_reg;

pub use crate::{
    item_spec_rt_id::ItemSpecRtId, resources::Resources, state_diffs::StateDiffs,
    state_diffs_mut::StateDiffsMut, states_current::StatesCurrent, states_desired::StatesDesired,
    states_ensured::StatesEnsured, states_ensured_dry::StatesEnsuredDry,
};

pub mod dir;
pub mod internal;
pub mod resources_type_state;
pub mod states;

mod item_spec_rt_id;
mod resources;
mod state_diffs;
mod state_diffs_mut;
mod states_current;
mod states_desired;
mod states_ensured;
mod states_ensured_dry;
