//! Copies a number from one resource to another.

pub use self::{
    iam_role_apply_fns::IamRoleApplyFns,
    iam_role_data::IamRoleData,
    iam_role_error::IamRoleError,
    iam_role_item::IamRoleItem,
    iam_role_params::{IamRoleParams, IamRoleParamsFieldWise, IamRoleParamsPartial},
    iam_role_state::IamRoleState,
    iam_role_state_current_fn::IamRoleStateCurrentFn,
    iam_role_state_diff::IamRoleStateDiff,
    iam_role_state_diff_fn::IamRoleStateDiffFn,
    iam_role_state_goal_fn::IamRoleStateGoalFn,
};

pub mod model;

mod iam_role_apply_fns;
mod iam_role_data;
mod iam_role_error;
mod iam_role_item;
mod iam_role_params;
mod iam_role_state;
mod iam_role_state_current_fn;
mod iam_role_state_diff;
mod iam_role_state_diff_fn;
mod iam_role_state_goal_fn;
