use std::marker::PhantomData;

use peace::cfg::{OpCheckStatus, OpCtx, State};

use crate::{
    ShSyncCmdData, ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdStateDiff,
    ShSyncCmdSyncStatus,
};

/// ApplyOpSpec for the command to execute.
#[derive(Debug)]
pub struct ShSyncCmdApplyOpSpec<Id>(PhantomData<Id>);

impl<Id> ShSyncCmdApplyOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn apply_check(
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _file_state_current: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<OpCheckStatus, ShSyncCmdError> {
        todo!();
    }

    pub async fn apply_dry(
        _op_ctx: OpCtx<'_>,
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>, ShSyncCmdError> {
        todo!()
    }

    pub async fn apply(
        _op_ctx: OpCtx<'_>,
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>, ShSyncCmdError> {
        todo!();
    }
}
