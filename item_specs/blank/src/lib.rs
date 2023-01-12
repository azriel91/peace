//! Copies a number from one resource to another.

pub use crate::{
    blank_clean_op_spec::BlankCleanOpSpec, blank_data::BlankData, blank_dest::BlankDest,
    blank_ensure_op_spec::BlankEnsureOpSpec, blank_error::BlankError,
    blank_item_spec::BlankItemSpec, blank_params::BlankParams, blank_src::BlankSrc,
    blank_state::BlankState, blank_state_current_fn_spec::BlankStateCurrentFnSpec,
    blank_state_desired_fn_spec::BlankStateDesiredFnSpec, blank_state_diff::BlankStateDiff,
    blank_state_diff_fn_spec::BlankStateDiffFnSpec,
};

mod blank_clean_op_spec;
mod blank_data;
mod blank_dest;
mod blank_ensure_op_spec;
mod blank_error;
mod blank_item_spec;
mod blank_params;
mod blank_src;
mod blank_state;
mod blank_state_current_fn_spec;
mod blank_state_desired_fn_spec;
mod blank_state_diff;
mod blank_state_diff_fn_spec;
