use std::marker::PhantomData;

use peace::cfg::{state::Timestamped, FnCtx};

use crate::item_specs::peace_aws_s3_bucket::{
    S3BucketData, S3BucketError, S3BucketParams, S3BucketState,
};

/// Reads the desired state of the S3 bucket state.
#[derive(Debug)]
pub struct S3BucketStateDesiredFn<Id>(PhantomData<Id>);

impl<Id> S3BucketStateDesiredFn<Id>
where
    Id: Send + Sync,
{
    pub async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&S3BucketParams<Id>>,
        data: S3BucketData<'_, Id>,
    ) -> Result<Option<S3BucketState>, S3BucketError> {
        if let Some(params) = params_partial {
            Self::state_desired(fn_ctx, params, data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    pub async fn state_desired(
        _fn_ctx: FnCtx<'_>,
        params: &S3BucketParams<Id>,
        _data: S3BucketData<'_, Id>,
    ) -> Result<S3BucketState, S3BucketError> {
        let name = params.name().to_string();

        Ok(S3BucketState::Some {
            name,
            creation_date: Timestamped::Tbd,
        })
    }
}
