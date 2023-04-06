use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId, OpCtx},
    resources::{resources::ts::Empty, Resources},
};

use crate::item_specs::peace_aws_s3_bucket::{
    S3BucketApplyOpSpec, S3BucketData, S3BucketError, S3BucketState, S3BucketStateCurrentFn,
    S3BucketStateDesiredFn, S3BucketStateDiff, S3BucketStateDiffFnSpec,
};

/// Item spec to create an IAM S3 bucket and IAM role.
///
/// In sequence, this will:
///
/// * Create the IAM Role.
/// * Create the S3 bucket.
/// * Add the IAM role to the S3 bucket.
///
/// The `Id` type parameter is needed for each S3 bucket params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different S3 bucket parameters
///   from each other.
#[derive(Debug)]
pub struct S3BucketItemSpec<Id> {
    /// ID of the S3 bucket item spec.
    item_spec_id: ItemSpecId,
    /// Marker for unique S3 bucket parameters type.
    marker: PhantomData<Id>,
}

impl<Id> S3BucketItemSpec<Id> {
    /// Returns a new `S3BucketItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for S3BucketItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for S3BucketItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type ApplyOpSpec = S3BucketApplyOpSpec<Id>;
    type Data<'op> = S3BucketData<'op, Id>;
    type Error = S3BucketError;
    type State = S3BucketState;
    type StateDiff = S3BucketStateDiff;
    type StateDiffFnSpec = S3BucketStateDiffFnSpec;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), S3BucketError> {
        if !resources.contains::<aws_sdk_s3::Client>() {
            let sdk_config = aws_config::load_from_env().await;
            resources.insert(sdk_config.region().cloned());
            let client = aws_sdk_s3::Client::new(&sdk_config);
            resources.insert(client);
        }
        Ok(())
    }

    async fn try_state_current(
        op_ctx: OpCtx<'_>,
        data: S3BucketData<'_, Id>,
    ) -> Result<Option<Self::State>, S3BucketError> {
        S3BucketStateCurrentFn::try_state_current(op_ctx, data).await
    }

    async fn state_current(
        op_ctx: OpCtx<'_>,
        data: S3BucketData<'_, Id>,
    ) -> Result<Self::State, S3BucketError> {
        S3BucketStateCurrentFn::state_current(op_ctx, data).await
    }

    async fn try_state_desired(
        op_ctx: OpCtx<'_>,
        data: S3BucketData<'_, Id>,
    ) -> Result<Option<Self::State>, S3BucketError> {
        S3BucketStateDesiredFn::try_state_desired(op_ctx, data).await
    }

    async fn state_desired(
        op_ctx: OpCtx<'_>,
        data: S3BucketData<'_, Id>,
    ) -> Result<Self::State, S3BucketError> {
        S3BucketStateDesiredFn::state_desired(op_ctx, data).await
    }

    async fn state_clean(_: Self::Data<'_>) -> Result<Self::State, S3BucketError> {
        Ok(S3BucketState::None)
    }
}
