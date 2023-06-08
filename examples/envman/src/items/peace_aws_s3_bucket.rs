//! Copies a number from one resource to another.

pub use self::{
    s3_bucket_apply_fns::S3BucketApplyFns,
    s3_bucket_data::S3BucketData,
    s3_bucket_error::S3BucketError,
    s3_bucket_item::S3BucketItem,
    s3_bucket_params::{S3BucketParams, S3BucketParamsFieldWise, S3BucketParamsPartial},
    s3_bucket_state::S3BucketState,
    s3_bucket_state_current_fn::S3BucketStateCurrentFn,
    s3_bucket_state_diff::S3BucketStateDiff,
    s3_bucket_state_diff_fn::S3BucketStateDiffFn,
    s3_bucket_state_goal_fn::S3BucketStateGoalFn,
};

mod s3_bucket_apply_fns;
mod s3_bucket_data;
mod s3_bucket_error;
mod s3_bucket_item;
mod s3_bucket_params;
mod s3_bucket_state;
mod s3_bucket_state_current_fn;
mod s3_bucket_state_diff;
mod s3_bucket_state_diff_fn;
mod s3_bucket_state_goal_fn;
