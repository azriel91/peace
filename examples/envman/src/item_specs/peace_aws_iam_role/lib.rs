//! Copies a number from one resource to another.

pub use crate::item_specs::peace_aws_iam_role::{
    iam_role_apply_op_spec::IamRoleApplyOpSpec, iam_role_data::IamRoleData,
    iam_role_error::IamRoleError, iam_role_item_spec::IamRoleItemSpec,
    iam_role_params::IamRoleParams, iam_role_state::IamRoleState,
    iam_role_state_current_fn::IamRoleStateCurrentFn,
    iam_role_state_desired_fn_spec::IamRoleStateDesiredFnSpec,
    iam_role_state_diff::IamRoleStateDiff, iam_role_state_diff_fn_spec::IamRoleStateDiffFnSpec,
};

pub mod model;

mod iam_role_apply_op_spec;
mod iam_role_data;
mod iam_role_error;
mod iam_role_item_spec;
mod iam_role_params;
mod iam_role_state;
mod iam_role_state_current_fn;
mod iam_role_state_desired_fn_spec;
mod iam_role_state_diff;
mod iam_role_state_diff_fn_spec;
