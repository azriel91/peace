//! Copies a number from one resource to another.

pub use self::{
    s3_bucket_apply_op_spec::S3BucketApplyOpSpec, s3_bucket_data::S3BucketData,
    s3_bucket_error::S3BucketError, s3_bucket_item_spec::S3BucketItemSpec,
    s3_bucket_params::S3BucketParams, s3_bucket_state::S3BucketState,
    s3_bucket_state_current_fn::S3BucketStateCurrentFn,
    s3_bucket_state_desired_fn::S3BucketStateDesiredFn, s3_bucket_state_diff::S3BucketStateDiff,
    s3_bucket_state_diff_fn_spec::S3BucketStateDiffFnSpec,
};

mod s3_bucket_apply_op_spec;
mod s3_bucket_data;
mod s3_bucket_error;
mod s3_bucket_item_spec;
mod s3_bucket_params;
mod s3_bucket_state;
mod s3_bucket_state_current_fn;
mod s3_bucket_state_desired_fn;
mod s3_bucket_state_diff;
mod s3_bucket_state_diff_fn_spec;
