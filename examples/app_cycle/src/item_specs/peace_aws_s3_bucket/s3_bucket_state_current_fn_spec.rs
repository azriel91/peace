use std::marker::PhantomData;

use aws_sdk_s3::{error::HeadBucketErrorKind, types::SdkError};
use peace::cfg::{async_trait, TryFnSpec};

use crate::item_specs::peace_aws_s3_bucket::{S3BucketData, S3BucketError, S3BucketState};

/// Reads the current state of the S3 bucket state.
#[derive(Debug)]
pub struct S3BucketStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for S3BucketStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = S3BucketData<'op, Id>;
    type Error = S3BucketError;
    type Output = S3BucketState;

    async fn try_exec(data: S3BucketData<'_, Id>) -> Result<Option<Self::Output>, S3BucketError> {
        Self::exec(data).await.map(Some)
    }

    async fn exec(data: S3BucketData<'_, Id>) -> Result<Self::Output, S3BucketError> {
        let client = data.client();
        let name = data.params().name();

        let head_bucket_result = client.head_bucket().bucket(name).send().await;
        let s3_bucket_exists = match head_bucket_result {
            Ok(_head_bucket_output) => true,
            Err(error) => match &error {
                SdkError::ServiceError(service_error) => match service_error.err().kind {
                    HeadBucketErrorKind::NotFound(_) => false,
                    _ => {
                        return Err(S3BucketError::S3BucketGetError {
                            s3_bucket_name: name.to_string(),
                            error,
                        });
                    }
                },
                _ => {
                    return Err(S3BucketError::S3BucketGetError {
                        s3_bucket_name: name.to_string(),
                        error,
                    });
                }
            },
        };

        if s3_bucket_exists {
            let state_current = S3BucketState::Some {
                name: name.to_string(),
            };

            Ok(state_current)
        } else {
            Ok(S3BucketState::None)
        }
    }
}
