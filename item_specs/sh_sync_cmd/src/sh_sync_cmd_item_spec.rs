use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId, OpCheckStatus, OpCtx, State},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    ShSyncCmdApplyOpSpec, ShSyncCmdData, ShSyncCmdError, ShSyncCmdExecutionRecord,
    ShSyncCmdStateDiff, ShSyncCmdStateDiffFn, ShSyncCmdSyncStatus,
};

/// Item spec for executing a shell command.
///
/// The `Id` type parameter is needed for each command execution params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Debug)]
pub struct ShSyncCmdItemSpec<Id> {
    /// ID to easily tell what the item spec command is for.
    item_spec_id: ItemSpecId,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> Clone for ShSyncCmdItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

impl<Id> ShSyncCmdItemSpec<Id> {
    /// Returns a new `ShSyncCmdItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for ShSyncCmdItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShSyncCmdData<'op, Id>;
    type Error = ShSyncCmdError;
    type State = State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>;
    type StateDiff = ShSyncCmdStateDiff;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, _resources: &mut Resources<Empty>) -> Result<(), ShSyncCmdError> {
        Ok(())
    }

    async fn try_state_current(
        op_ctx: OpCtx<'_>,
        data: ShSyncCmdData<'_, Id>,
    ) -> Result<Option<Self::State>, ShSyncCmdError> {
        Self::state_current(op_ctx, data).await.map(Some)
    }

    async fn state_current(
        _op_ctx: OpCtx<'_>,
        _data: ShSyncCmdData<'_, Id>,
    ) -> Result<Self::State, ShSyncCmdError> {
        todo!()
    }

    async fn try_state_desired(
        op_ctx: OpCtx<'_>,
        data: ShSyncCmdData<'_, Id>,
    ) -> Result<Option<Self::State>, ShSyncCmdError> {
        Self::state_desired(op_ctx, data).await.map(Some)
    }

    async fn state_desired(
        _op_ctx: OpCtx<'_>,
        _data: ShSyncCmdData<'_, Id>,
    ) -> Result<Self::State, ShSyncCmdError> {
        todo!()
    }

    async fn state_diff(
        data: ShSyncCmdData<'_, Id>,
        state_current: &Self::State,
        state_desired: &Self::State,
    ) -> Result<Self::StateDiff, ShSyncCmdError> {
        ShSyncCmdStateDiffFn::state_diff(data, state_current, state_desired).await
    }

    async fn state_clean(_: Self::Data<'_>) -> Result<Self::State, ShSyncCmdError> {
        let state = State::new(
            ShSyncCmdSyncStatus::NotExecuted,
            ShSyncCmdExecutionRecord::None,
        );
        Ok(state)
    }

    async fn apply_check(
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<OpCheckStatus, Self::Error> {
        ShSyncCmdApplyOpSpec::apply_check(data, state_current, state_target, diff).await
    }

    async fn apply_dry(
        op_ctx: OpCtx<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        ShSyncCmdApplyOpSpec::apply_dry(op_ctx, data, state_current, state_target, diff).await
    }

    async fn apply(
        op_ctx: OpCtx<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        ShSyncCmdApplyOpSpec::apply(op_ctx, data, state_current, state_target, diff).await
    }
}
