use std::marker::PhantomData;

use aws_config::BehaviorVersion;
use peace::{
    cfg::{async_trait, ApplyCheck, FnCtx, Item, ItemId},
    params::Params,
    resources::{resources::ts::Empty, Resources},
};

use crate::items::peace_aws_iam_role::{
    IamRoleApplyFns, IamRoleData, IamRoleError, IamRoleParams, IamRoleState, IamRoleStateCurrentFn,
    IamRoleStateDiff, IamRoleStateDiffFn, IamRoleStateGoalFn,
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
pub struct IamRoleItem<Id> {
    /// ID of the instance profile item.
    item_id: ItemId,
    /// Marker for unique instance profile parameters type.
    marker: PhantomData<Id>,
}

impl<Id> IamRoleItem<Id> {
    /// Returns a new `IamRoleItem`.
    pub fn new(item_id: ItemId) -> Self {
        Self {
            item_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for IamRoleItem<Id> {
    fn clone(&self) -> Self {
        Self {
            item_id: self.item_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> Item for IamRoleItem<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'exec> = IamRoleData<'exec, Id>;
    type Error = IamRoleError;
    type Params<'exec> = IamRoleParams<Id>;
    type State = IamRoleState;
    type StateDiff = IamRoleStateDiff;

    fn id(&self) -> &ItemId {
        &self.item_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), IamRoleError> {
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
        data: IamRoleData<'_, Id>,
    ) -> Result<Option<Self::State>, IamRoleError> {
        IamRoleStateCurrentFn::try_state_current(fn_ctx, params_partial, data).await
    }

    async fn state_current(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: IamRoleData<'_, Id>,
    ) -> Result<Self::State, IamRoleError> {
        IamRoleStateCurrentFn::state_current(fn_ctx, params, data).await
    }

    async fn try_state_goal(
        fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Params>::Partial,
        data: IamRoleData<'_, Id>,
    ) -> Result<Option<Self::State>, IamRoleError> {
        IamRoleStateGoalFn::try_state_goal(fn_ctx, params_partial, data).await
    }

    async fn state_goal(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: IamRoleData<'_, Id>,
    ) -> Result<Self::State, IamRoleError> {
        IamRoleStateGoalFn::state_goal(fn_ctx, params, data).await
    }

    async fn state_diff(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
        state_current: &Self::State,
        state_goal: &Self::State,
    ) -> Result<Self::StateDiff, IamRoleError> {
        IamRoleStateDiffFn::state_diff(state_current, state_goal).await
    }

    async fn state_clean(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
    ) -> Result<Self::State, IamRoleError> {
        Ok(IamRoleState::None)
    }

    async fn apply_check(
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<ApplyCheck, Self::Error> {
        IamRoleApplyFns::<Id>::apply_check(params, data, state_current, state_target, diff).await
    }

    async fn apply_dry(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        IamRoleApplyFns::<Id>::apply_dry(fn_ctx, params, data, state_current, state_target, diff)
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
        IamRoleApplyFns::<Id>::apply(fn_ctx, params, data, state_current, state_target, diff).await
    }
}
