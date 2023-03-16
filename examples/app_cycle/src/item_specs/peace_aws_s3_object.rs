//! Uploads a file to S3.

pub use self::{
    s3_object_clean_op_spec::S3ObjectCleanOpSpec, s3_object_data::S3ObjectData,
    s3_object_ensure_op_spec::S3ObjectEnsureOpSpec, s3_object_error::S3ObjectError,
    s3_object_item_spec::S3ObjectItemSpec, s3_object_params::S3ObjectParams,
    s3_object_state::S3ObjectState, s3_object_state_current_fn_spec::S3ObjectStateCurrentFnSpec,
    s3_object_state_desired_fn_spec::S3ObjectStateDesiredFnSpec,
    s3_object_state_diff::S3ObjectStateDiff, s3_object_state_diff_fn_spec::S3ObjectStateDiffFnSpec,
};

mod s3_object_clean_op_spec;
mod s3_object_data;
mod s3_object_ensure_op_spec;
mod s3_object_error;
mod s3_object_item_spec;
mod s3_object_params;
mod s3_object_state;
mod s3_object_state_current_fn_spec;
mod s3_object_state_desired_fn_spec;
mod s3_object_state_diff;
mod s3_object_state_diff_fn_spec;
