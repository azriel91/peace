//! Copies a number from one resource to another.

pub use self::{
    instance_profile_apply_fns::InstanceProfileApplyFns,
    instance_profile_data::InstanceProfileData,
    instance_profile_error::InstanceProfileError,
    instance_profile_item_spec::InstanceProfileItemSpec,
    instance_profile_params::{
        InstanceProfileParams, InstanceProfileParamsPartial, InstanceProfileParamsSpec,
    },
    instance_profile_state::InstanceProfileState,
    instance_profile_state_current_fn::InstanceProfileStateCurrentFn,
    instance_profile_state_desired_fn::InstanceProfileStateDesiredFn,
    instance_profile_state_diff::InstanceProfileStateDiff,
    instance_profile_state_diff_fn::InstanceProfileStateDiffFn,
};

pub mod model;

mod instance_profile_apply_fns;
mod instance_profile_data;
mod instance_profile_error;
mod instance_profile_item_spec;
mod instance_profile_params;
mod instance_profile_state;
mod instance_profile_state_current_fn;
mod instance_profile_state_desired_fn;
mod instance_profile_state_diff;
mod instance_profile_state_diff_fn;
