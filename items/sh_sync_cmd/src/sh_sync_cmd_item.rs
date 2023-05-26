use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ApplyCheck, FnCtx, Item, ItemId, State},
    params::Params,
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    ShSyncCmdApplyFns, ShSyncCmdData, ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdParams,
    ShSyncCmdStateDiff, ShSyncCmdStateDiffFn, ShSyncCmdSyncStatus,
};

/// Item for executing a shell command.
///
/// The `Id` type parameter is needed for each command execution params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Debug)]
pub struct ShSyncCmdItem<Id> {
    /// ID to easily tell what the item command is for.
    item_id: ItemId,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> Clone for ShSyncCmdItem<Id> {
    fn clone(&self) -> Self {
        Self {
            item_id: self.item_id.clone(),
            marker: PhantomData,
        }
    }
}

impl<Id> ShSyncCmdItem<Id> {
    /// Returns a new `ShSyncCmdItem`.
    pub fn new(item_id: ItemId) -> Self {
        Self {
            item_id,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> Item for ShSyncCmdItem<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'exec> = ShSyncCmdData<'exec, Id>;
    type Error = ShSyncCmdError;
    type Params<'exec> = ShSyncCmdParams<Id>;
    type State = State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>;
    type StateDiff = ShSyncCmdStateDiff;

    fn id(&self) -> &ItemId {
        &self.item_id
    }

    async fn setup(&self, _resources: &mut Resources<Empty>) -> Result<(), ShSyncCmdError> {
        Ok(())
    }

    async fn try_state_current(
        _fn_ctx: FnCtx<'_>,
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: ShSyncCmdData<'_, Id>,
    ) -> Result<Option<Self::State>, ShSyncCmdError> {
        todo!()
    }

    async fn state_current(
        _fn_ctx: FnCtx<'_>,
        _params: &Self::Params<'_>,
        _data: ShSyncCmdData<'_, Id>,
    ) -> Result<Self::State, ShSyncCmdError> {
        todo!()
    }

    async fn try_state_desired(
        _fn_ctx: FnCtx<'_>,
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: ShSyncCmdData<'_, Id>,
    ) -> Result<Option<Self::State>, ShSyncCmdError> {
        todo!()
    }

    async fn state_desired(
        _fn_ctx: FnCtx<'_>,
        _params: &Self::Params<'_>,
        _data: ShSyncCmdData<'_, Id>,
    ) -> Result<Self::State, ShSyncCmdError> {
        todo!()
    }

    async fn state_diff(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: ShSyncCmdData<'_, Id>,
        state_current: &Self::State,
        state_desired: &Self::State,
    ) -> Result<Self::StateDiff, ShSyncCmdError> {
        ShSyncCmdStateDiffFn::<Id>::state_diff(state_current, state_desired).await
    }

    async fn state_clean(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
    ) -> Result<Self::State, ShSyncCmdError> {
        let state = State::new(
            ShSyncCmdSyncStatus::NotExecuted,
            ShSyncCmdExecutionRecord::None,
        );
        Ok(state)
    }

    async fn apply_check(
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<ApplyCheck, Self::Error> {
        ShSyncCmdApplyFns::<Id>::apply_check(params, data, state_current, state_target, diff).await
    }

    async fn apply_dry(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        ShSyncCmdApplyFns::<Id>::apply_dry(fn_ctx, params, data, state_current, state_target, diff)
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
        ShSyncCmdApplyFns::<Id>::apply(fn_ctx, params, data, state_current, state_target, diff)
            .await
    }
}
