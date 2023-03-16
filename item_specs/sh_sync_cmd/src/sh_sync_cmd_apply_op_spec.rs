use std::marker::PhantomData;

use peace::cfg::{async_trait, ApplyOpSpec, OpCheckStatus, OpCtx, State};

use crate::{
    ShSyncCmdData, ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdStateDiff,
    ShSyncCmdSyncStatus,
};

/// ApplyOpSpec for the command to execute.
#[derive(Debug)]
pub struct ShSyncCmdApplyOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> ApplyOpSpec for ShSyncCmdApplyOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShSyncCmdData<'op, Id>;
    type Error = ShSyncCmdError;
    type State = State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>;
    type StateDiff = ShSyncCmdStateDiff;

    async fn check(
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _file_state_current: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<OpCheckStatus, ShSyncCmdError> {
        todo!();
    }

    async fn exec_dry(
        _op_ctx: OpCtx<'_>,
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<Self::State, ShSyncCmdError> {
        todo!()
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<Self::State, ShSyncCmdError> {
        todo!();
    }
}
