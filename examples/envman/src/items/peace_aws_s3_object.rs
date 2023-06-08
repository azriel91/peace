//! Uploads a file to S3.

pub use self::{
    s3_object_apply_fns::S3ObjectApplyFns,
    s3_object_data::S3ObjectData,
    s3_object_error::S3ObjectError,
    s3_object_item::S3ObjectItem,
    s3_object_params::{S3ObjectParams, S3ObjectParamsFieldWise, S3ObjectParamsPartial},
    s3_object_state::S3ObjectState,
    s3_object_state_current_fn::S3ObjectStateCurrentFn,
    s3_object_state_diff::S3ObjectStateDiff,
    s3_object_state_diff_fn::S3ObjectStateDiffFn,
    s3_object_state_goal_fn::S3ObjectStateGoalFn,
};

mod s3_object_apply_fns;
mod s3_object_data;
mod s3_object_error;
mod s3_object_item;
mod s3_object_params;
mod s3_object_state;
mod s3_object_state_current_fn;
mod s3_object_state_diff;
mod s3_object_state_diff_fn;
mod s3_object_state_goal_fn;
