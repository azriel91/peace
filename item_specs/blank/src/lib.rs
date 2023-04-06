//! Copies a number from one resource to another.

pub use crate::{
    blank_apply_op_spec::BlankApplyOpSpec, blank_data::BlankData, blank_dest::BlankDest,
    blank_error::BlankError, blank_item_spec::BlankItemSpec, blank_params::BlankParams,
    blank_src::BlankSrc, blank_state::BlankState, blank_state_diff::BlankStateDiff,
};

mod blank_apply_op_spec;
mod blank_data;
mod blank_dest;
mod blank_error;
mod blank_item_spec;
mod blank_params;
mod blank_src;
mod blank_state;
mod blank_state_diff;
