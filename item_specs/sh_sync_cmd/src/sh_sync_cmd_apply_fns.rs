use std::marker::PhantomData;

use peace::cfg::{FnCtx, OpCheckStatus, State};

use crate::{
    ShSyncCmdData, ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdStateDiff,
    ShSyncCmdSyncStatus,
};

/// ApplyFns for the command to execute.
#[derive(Debug)]
pub struct ShSyncCmdApplyFns<Id>(PhantomData<Id>);

impl<Id> ShSyncCmdApplyFns<Id>
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
        _fn_ctx: FnCtx<'_>,
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>, ShSyncCmdError> {
        todo!()
    }

    pub async fn apply(
        _fn_ctx: FnCtx<'_>,
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>, ShSyncCmdError> {
        todo!();
    }
}
