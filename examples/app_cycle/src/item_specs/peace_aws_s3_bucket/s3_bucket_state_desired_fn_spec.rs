use std::marker::PhantomData;

use peace::cfg::{async_trait, TryFnSpec};

use crate::item_specs::peace_aws_s3_bucket::{S3BucketData, S3BucketError, S3BucketState};

/// Reads the desired state of the S3 bucket state.
#[derive(Debug)]
pub struct S3BucketStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for S3BucketStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = S3BucketData<'op, Id>;
    type Error = S3BucketError;
    type Output = S3BucketState;

    async fn try_exec(
        s3_bucket_data: S3BucketData<'_, Id>,
    ) -> Result<Option<Self::Output>, S3BucketError> {
        Self::exec(s3_bucket_data).await.map(Some)
    }

    async fn exec(s3_bucket_data: S3BucketData<'_, Id>) -> Result<Self::Output, S3BucketError> {
        let params = s3_bucket_data.params();
        let name = params.name().to_string();

        Ok(S3BucketState::Some { name })
    }
}
