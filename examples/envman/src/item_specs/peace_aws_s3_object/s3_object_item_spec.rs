use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ApplyCheck, FnCtx, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::item_specs::peace_aws_s3_object::{
    S3ObjectApplyFns, S3ObjectData, S3ObjectError, S3ObjectState, S3ObjectStateCurrentFn,
    S3ObjectStateDesiredFn, S3ObjectStateDiff, S3ObjectStateDiffFn,
};

/// Item spec to create an IAM S3 object and IAM role.
///
/// In sequence, this will:
///
/// * Create the IAM Role.
/// * Create the S3 object.
/// * Add the IAM role to the S3 object.
///
/// The `Id` type parameter is needed for each S3 object params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different S3 object parameters
///   from each other.
#[derive(Debug)]
pub struct S3ObjectItemSpec<Id> {
    /// ID of the S3 object item spec.
    item_spec_id: ItemSpecId,
    /// Marker for unique S3 object parameters type.
    marker: PhantomData<Id>,
}

impl<Id> S3ObjectItemSpec<Id> {
    /// Returns a new `S3ObjectItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for S3ObjectItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for S3ObjectItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = S3ObjectData<'op, Id>;
    type Error = S3ObjectError;
    type State = S3ObjectState;
    type StateDiff = S3ObjectStateDiff;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), S3ObjectError> {
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
        data: S3ObjectData<'_, Id>,
    ) -> Result<Option<Self::State>, S3ObjectError> {
        S3ObjectStateCurrentFn::try_state_current(fn_ctx, data).await
    }

    async fn state_current(
        fn_ctx: FnCtx<'_>,
        data: S3ObjectData<'_, Id>,
    ) -> Result<Self::State, S3ObjectError> {
        S3ObjectStateCurrentFn::state_current(fn_ctx, data).await
    }

    async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        data: S3ObjectData<'_, Id>,
    ) -> Result<Option<Self::State>, S3ObjectError> {
        S3ObjectStateDesiredFn::try_state_desired(fn_ctx, data).await
    }

    async fn state_desired(
        fn_ctx: FnCtx<'_>,
        data: S3ObjectData<'_, Id>,
    ) -> Result<Self::State, S3ObjectError> {
        S3ObjectStateDesiredFn::state_desired(fn_ctx, data).await
    }

    async fn state_diff(
        _data: S3ObjectData<'_, Id>,
        state_current: &Self::State,
        state_desired: &Self::State,
    ) -> Result<Self::StateDiff, S3ObjectError> {
        S3ObjectStateDiffFn::state_diff(state_current, state_desired).await
    }

    async fn state_clean(_: Self::Data<'_>) -> Result<Self::State, S3ObjectError> {
        Ok(S3ObjectState::None)
    }

    async fn apply_check(
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<ApplyCheck, Self::Error> {
        S3ObjectApplyFns::apply_check(data, state_current, state_target, diff).await
    }

    async fn apply_dry(
        fn_ctx: FnCtx<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        S3ObjectApplyFns::apply_dry(fn_ctx, data, state_current, state_target, diff).await
    }

    async fn apply(
        fn_ctx: FnCtx<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        S3ObjectApplyFns::apply(fn_ctx, data, state_current, state_target, diff).await
    }
}
