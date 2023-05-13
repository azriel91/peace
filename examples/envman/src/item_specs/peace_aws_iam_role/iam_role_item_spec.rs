use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ApplyCheck, FnCtx, ItemSpec, ItemSpecId},
    params::Value,
    resources::{resources::ts::Empty, Resources},
};

use crate::item_specs::peace_aws_iam_role::{
    IamRoleApplyFns, IamRoleData, IamRoleError, IamRoleParams, IamRoleState, IamRoleStateCurrentFn,
    IamRoleStateDesiredFn, IamRoleStateDiff, IamRoleStateDiffFn,
};

/// Item spec to create an IAM instance profile and IAM role.
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
pub struct IamRoleItemSpec<Id> {
    /// ID of the instance profile item spec.
    item_spec_id: ItemSpecId,
    /// Marker for unique instance profile parameters type.
    marker: PhantomData<Id>,
}

impl<Id> IamRoleItemSpec<Id> {
    /// Returns a new `IamRoleItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for IamRoleItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for IamRoleItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'exec> = IamRoleData<'exec, Id>;
    type Error = IamRoleError;
    type Params<'exec> = IamRoleParams<Id>;
    type State = IamRoleState;
    type StateDiff = IamRoleStateDiff;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), IamRoleError> {
        if !resources.contains::<aws_sdk_iam::Client>() {
            let sdk_config = aws_config::load_from_env().await;
            let client = aws_sdk_iam::Client::new(&sdk_config);
            resources.insert(client);
        }
        Ok(())
    }

    async fn try_state_current(
        fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Value>::Partial,
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

    async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Value>::Partial,
        data: IamRoleData<'_, Id>,
    ) -> Result<Option<Self::State>, IamRoleError> {
        IamRoleStateDesiredFn::try_state_desired(fn_ctx, params_partial, data).await
    }

    async fn state_desired(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: IamRoleData<'_, Id>,
    ) -> Result<Self::State, IamRoleError> {
        IamRoleStateDesiredFn::state_desired(fn_ctx, params, data).await
    }

    async fn state_diff(
        _params_partial: &<Self::Params<'_> as Value>::Partial,
        _data: Self::Data<'_>,
        state_current: &Self::State,
        state_desired: &Self::State,
    ) -> Result<Self::StateDiff, IamRoleError> {
        IamRoleStateDiffFn::state_diff(state_current, state_desired).await
    }

    async fn state_clean(
        _params_partial: &<Self::Params<'_> as Value>::Partial,
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
