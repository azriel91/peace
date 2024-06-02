use std::marker::PhantomData;

use aws_config::BehaviorVersion;
use peace::{
    cfg::{async_trait, ApplyCheck, FnCtx, Item, ItemId},
    params::Params,
    resources::{resources::ts::Empty, Resources},
};

use crate::items::peace_aws_instance_profile::{
    InstanceProfileApplyFns, InstanceProfileData, InstanceProfileError, InstanceProfileParams,
    InstanceProfileState, InstanceProfileStateCurrentFn, InstanceProfileStateDiff,
    InstanceProfileStateDiffFn, InstanceProfileStateGoalFn,
};

/// Item to create an IAM instance profile and IAM role.
///
/// In sequence, this will:
///
/// * Create the IAM Role.
/// * Create the instance profile.
/// * Add the IAM role to the instance profile.
///
/// The `Id` type parameter is needed for each instance profile params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different instance profile
///   parameters from each other.
#[derive(Debug)]
pub struct InstanceProfileItem<Id> {
    /// ID of the instance profile item.
    item_id: ItemId,
    /// Marker for unique instance profile parameters type.
    marker: PhantomData<Id>,
}

impl<Id> InstanceProfileItem<Id> {
    /// Returns a new `InstanceProfileItem`.
    pub fn new(item_id: ItemId) -> Self {
        Self {
            item_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for InstanceProfileItem<Id> {
    fn clone(&self) -> Self {
        Self {
            item_id: self.item_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> Item for InstanceProfileItem<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'exec> = InstanceProfileData<'exec, Id>;
    type Error = InstanceProfileError;
    type Params<'exec> = InstanceProfileParams<Id>;
    type State = InstanceProfileState;
    type StateDiff = InstanceProfileStateDiff;

    fn id(&self) -> &ItemId {
        &self.item_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), InstanceProfileError> {
        if !resources.contains::<aws_sdk_iam::Client>() {
            let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
            let client = aws_sdk_iam::Client::new(&sdk_config);
            resources.insert(client);
        }
        Ok(())
    }

    async fn try_state_current(
        fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Params>::Partial,
        data: InstanceProfileData<'_, Id>,
    ) -> Result<Option<Self::State>, InstanceProfileError> {
        InstanceProfileStateCurrentFn::try_state_current(fn_ctx, params_partial, data).await
    }

    async fn state_current(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: InstanceProfileData<'_, Id>,
    ) -> Result<Self::State, InstanceProfileError> {
        InstanceProfileStateCurrentFn::state_current(fn_ctx, params, data).await
    }

    async fn try_state_goal(
        fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Params>::Partial,
        data: InstanceProfileData<'_, Id>,
    ) -> Result<Option<Self::State>, InstanceProfileError> {
        InstanceProfileStateGoalFn::try_state_goal(fn_ctx, params_partial, data).await
    }

    async fn state_goal(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: InstanceProfileData<'_, Id>,
    ) -> Result<Self::State, InstanceProfileError> {
        InstanceProfileStateGoalFn::state_goal(fn_ctx, params, data).await
    }

    async fn state_diff(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
        state_current: &Self::State,
        state_goal: &Self::State,
    ) -> Result<Self::StateDiff, InstanceProfileError> {
        InstanceProfileStateDiffFn::state_diff(state_current, state_goal).await
    }

    async fn state_clean(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
    ) -> Result<Self::State, InstanceProfileError> {
        Ok(InstanceProfileState::None)
    }

    async fn apply_check(
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<ApplyCheck, Self::Error> {
        InstanceProfileApplyFns::<Id>::apply_check(params, data, state_current, state_target, diff)
            .await
    }

    async fn apply_dry(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        InstanceProfileApplyFns::<Id>::apply_dry(
            fn_ctx,
            params,
            data,
            state_current,
            state_target,
            diff,
        )
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
        InstanceProfileApplyFns::<Id>::apply(
            fn_ctx,
            params,
            data,
            state_current,
            state_target,
            diff,
        )
        .await
    }
}
