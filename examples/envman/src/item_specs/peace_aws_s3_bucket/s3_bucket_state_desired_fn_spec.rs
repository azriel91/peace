use std::marker::PhantomData;

use peace::cfg::{state::Timestamped, OpCtx};

use crate::item_specs::peace_aws_s3_bucket::{S3BucketData, S3BucketError, S3BucketState};

/// Reads the desired state of the S3 bucket state.
#[derive(Debug)]
pub struct S3BucketStateDesiredFnSpec<Id>(PhantomData<Id>);

impl<Id> S3BucketStateDesiredFnSpec<Id>
where
    Id: Send + Sync,
{
    pub async fn try_state_desired(
        op_ctx: OpCtx<'_>,
        data: S3BucketData<'_, Id>,
    ) -> Result<Option<S3BucketState>, S3BucketError> {
        Self::state_desired(op_ctx, data).await.map(Some)
    }

    pub async fn state_desired(
        _op_ctx: OpCtx<'_>,
        data: S3BucketData<'_, Id>,
    ) -> Result<S3BucketState, S3BucketError> {
        let params = data.params();
        let name = params.name().to_string();

        Ok(S3BucketState::Some {
            name,
            creation_date: Timestamped::Tbd,
        })
    }
}
