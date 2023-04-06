use crate::item_specs::peace_aws_s3_bucket::{S3BucketError, S3BucketState, S3BucketStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct S3BucketStateDiffFnSpec;

impl S3BucketStateDiffFnSpec {
    pub async fn state_diff(
        state_current: &S3BucketState,
        state_desired: &S3BucketState,
    ) -> Result<S3BucketStateDiff, S3BucketError> {
        let diff = match (state_current, state_desired) {
            (S3BucketState::None, S3BucketState::None) => S3BucketStateDiff::InSyncDoesNotExist,
            (S3BucketState::None, S3BucketState::Some { .. }) => S3BucketStateDiff::Added,
            (S3BucketState::Some { .. }, S3BucketState::None) => S3BucketStateDiff::Removed,
            (
                S3BucketState::Some {
                    name: s3_bucket_name_current,
                    creation_date: _,
                },
                S3BucketState::Some {
                    name: s3_bucket_name_desired,
                    creation_date: _,
                },
            ) => {
                if s3_bucket_name_current != s3_bucket_name_desired {
                    S3BucketStateDiff::NameModified {
                        s3_bucket_name_current: s3_bucket_name_current.to_string(),
                        s3_bucket_name_desired: s3_bucket_name_desired.to_string(),
                    }
                } else {
                    // We don't care about the creation date, as existence is sufficient.
                    S3BucketStateDiff::InSyncExists
                }
            }
        };

        Ok(diff)
    }
}
