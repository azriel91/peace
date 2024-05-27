use crate::items::peace_aws_s3_object::{S3ObjectError, S3ObjectState, S3ObjectStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct S3ObjectStateDiffFn;

impl S3ObjectStateDiffFn {
    pub async fn state_diff(
        state_current: &S3ObjectState,
        state_goal: &S3ObjectState,
    ) -> Result<S3ObjectStateDiff, S3ObjectError> {
        let diff = match (state_current, state_goal) {
            (S3ObjectState::None, S3ObjectState::None) => S3ObjectStateDiff::InSyncDoesNotExist,
            (S3ObjectState::None, S3ObjectState::Some { .. }) => S3ObjectStateDiff::Added,
            (S3ObjectState::Some { .. }, S3ObjectState::None) => S3ObjectStateDiff::Removed,
            (
                S3ObjectState::Some {
                    bucket_name: bucket_name_current,
                    object_key: object_key_current,
                    content_md5_hexstr: content_md5_hexstr_current,
                    e_tag: _e_tag_current,
                },
                S3ObjectState::Some {
                    bucket_name: bucket_name_goal,
                    object_key: object_key_goal,
                    content_md5_hexstr: content_md5_hexstr_goal,
                    e_tag: _e_tag_goal,
                },
            ) => {
                if bucket_name_current != bucket_name_goal {
                    S3ObjectStateDiff::BucketNameModified {
                        bucket_name_current: bucket_name_current.to_string(),
                        bucket_name_goal: bucket_name_goal.to_string(),
                    }
                } else if object_key_current != object_key_goal {
                    S3ObjectStateDiff::ObjectKeyModified {
                        object_key_current: object_key_current.to_string(),
                        object_key_goal: object_key_goal.to_string(),
                    }
                } else if content_md5_hexstr_current != content_md5_hexstr_goal {
                    S3ObjectStateDiff::ObjectContentModified {
                        content_md5_hexstr_current: content_md5_hexstr_current.clone(),
                        content_md5_hexstr_goal: content_md5_hexstr_goal.clone(),
                    }
                } else {
                    S3ObjectStateDiff::InSyncExists
                }
            }
        };

        Ok(diff)
    }
}
