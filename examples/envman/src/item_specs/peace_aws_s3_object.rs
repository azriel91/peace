//! Uploads a file to S3.

pub use self::{
    s3_object_apply_fns::S3ObjectApplyFns,
    s3_object_data::S3ObjectData,
    s3_object_error::S3ObjectError,
    s3_object_item_spec::S3ObjectItemSpec,
    s3_object_params::{S3ObjectParams, S3ObjectParamsPartial, S3ObjectParamsSpec},
    s3_object_state::S3ObjectState,
    s3_object_state_current_fn::S3ObjectStateCurrentFn,
    s3_object_state_desired_fn::S3ObjectStateDesiredFn,
    s3_object_state_diff::S3ObjectStateDiff,
    s3_object_state_diff_fn::S3ObjectStateDiffFn,
};

mod s3_object_apply_fns;
mod s3_object_data;
mod s3_object_error;
mod s3_object_item_spec;
mod s3_object_params;
mod s3_object_state;
mod s3_object_state_current_fn;
mod s3_object_state_desired_fn;
mod s3_object_state_diff;
mod s3_object_state_diff_fn;
