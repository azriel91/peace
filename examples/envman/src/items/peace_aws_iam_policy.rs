//! Copies a number from one resource to another.

pub use self::{
    iam_policy_apply_fns::IamPolicyApplyFns,
    iam_policy_data::IamPolicyData,
    iam_policy_error::IamPolicyError,
    iam_policy_item::IamPolicyItem,
    iam_policy_params::{IamPolicyParams, IamPolicyParamsFieldWise, IamPolicyParamsPartial},
    iam_policy_state::IamPolicyState,
    iam_policy_state_current_fn::IamPolicyStateCurrentFn,
    iam_policy_state_diff::IamPolicyStateDiff,
    iam_policy_state_diff_fn::IamPolicyStateDiffFn,
    iam_policy_state_goal_fn::IamPolicyStateGoalFn,
};

pub mod model;

mod iam_policy_apply_fns;
mod iam_policy_data;
mod iam_policy_error;
mod iam_policy_item;
mod iam_policy_params;
mod iam_policy_state;
mod iam_policy_state_current_fn;
mod iam_policy_state_diff;
mod iam_policy_state_diff_fn;
mod iam_policy_state_goal_fn;
