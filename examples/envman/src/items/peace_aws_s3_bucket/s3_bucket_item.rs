use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ApplyCheck, FnCtx, Item, ItemId},
    params::Params,
    resources::{resources::ts::Empty, Resources},
};

use crate::items::peace_aws_s3_bucket::{
    S3BucketApplyFns, S3BucketData, S3BucketError, S3BucketParams, S3BucketState,
    S3BucketStateCurrentFn, S3BucketStateDesiredFn, S3BucketStateDiff, S3BucketStateDiffFn,
};

/// Item to create an IAM S3 bucket and IAM role.
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
pub struct S3BucketItem<Id> {
    /// ID of the S3 bucket item.
    item_id: ItemId,
    /// Marker for unique S3 bucket parameters type.
    marker: PhantomData<Id>,
}

impl<Id> S3BucketItem<Id> {
    /// Returns a new `S3BucketItem`.
    pub fn new(item_id: ItemId) -> Self {
        Self {
            item_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for S3BucketItem<Id> {
    fn clone(&self) -> Self {
        Self {
            item_id: self.item_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> Item for S3BucketItem<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'exec> = S3BucketData<'exec, Id>;
    type Error = S3BucketError;
    type Params<'exec> = S3BucketParams<Id>;
    type State = S3BucketState;
    type StateDiff = S3BucketStateDiff;

    fn id(&self) -> &ItemId {
        &self.item_id
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
        fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Params>::Partial,
        data: Self::Data<'_>,
    ) -> Result<Option<Self::State>, S3BucketError> {
        S3BucketStateCurrentFn::try_state_current(fn_ctx, params_partial, data).await
    }

    async fn state_current(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
    ) -> Result<Self::State, S3BucketError> {
        S3BucketStateCurrentFn::state_current(fn_ctx, params, data).await
    }

    async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Params>::Partial,
        data: Self::Data<'_>,
    ) -> Result<Option<Self::State>, S3BucketError> {
        S3BucketStateDesiredFn::try_state_desired(fn_ctx, params_partial, data).await
    }

    async fn state_desired(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
    ) -> Result<Self::State, S3BucketError> {
        S3BucketStateDesiredFn::state_desired(fn_ctx, params, data).await
    }

    async fn state_diff(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
        state_current: &Self::State,
        state_desired: &Self::State,
    ) -> Result<Self::StateDiff, S3BucketError> {
        S3BucketStateDiffFn::state_diff(state_current, state_desired).await
    }

    async fn state_clean(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
    ) -> Result<Self::State, S3BucketError> {
        Ok(S3BucketState::None)
    }

    async fn apply_check(
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<ApplyCheck, Self::Error> {
        S3BucketApplyFns::<Id>::apply_check(params, data, state_current, state_target, diff).await
    }

    async fn apply_dry(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        S3BucketApplyFns::<Id>::apply_dry(fn_ctx, params, data, state_current, state_target, diff)
            .await
    }

    async fn apply(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        S3BucketApplyFns::<Id>::apply(fn_ctx, params, data, state_current, state_target, diff).await
    }
}
