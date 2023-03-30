use peace::cfg::{async_trait, StateDiffFnSpec};

use crate::item_specs::peace_aws_s3_bucket::{S3BucketError, S3BucketState, S3BucketStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct S3BucketStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for S3BucketStateDiffFnSpec {
    type Data<'op> = &'op ();
    type Error = S3BucketError;
    type State = S3BucketState;
    type StateDiff = S3BucketStateDiff;

    async fn exec(
        _: &(),
        state_current: &S3BucketState,
        state_desired: &S3BucketState,
    ) -> Result<Self::StateDiff, S3BucketError> {
        let diff = match (state_current, state_desired) {
            (S3BucketState::None, S3BucketState::None) => S3BucketStateDiff::InSyncDoesNotExist,
            (S3BucketState::None, S3BucketState::Some { .. }) => S3BucketStateDiff::Added,
            (S3BucketState::Some { .. }, S3BucketState::None) => S3BucketStateDiff::Removed,
            (
                S3BucketState::Some {
                    name: s3_bucket_name_current,
                },
                S3BucketState::Some {
                    name: s3_bucket_name_desired,
                },
            ) => {
                if s3_bucket_name_current != s3_bucket_name_desired {
                    S3BucketStateDiff::NameModified {
                        s3_bucket_name_current: s3_bucket_name_current.to_string(),
                        s3_bucket_name_desired: s3_bucket_name_desired.to_string(),
                    }
                } else {
                    S3BucketStateDiff::InSyncExists
                }
            }
        };

        Ok(diff)
    }
}
