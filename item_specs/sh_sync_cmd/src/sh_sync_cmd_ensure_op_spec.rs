use std::marker::PhantomData;

use peace::cfg::{async_trait, EnsureOpSpec, OpCheckStatus, State};

use crate::{
    ShSyncCmdData, ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdStateDiff,
    ShSyncCmdSyncStatus,
};

/// Ensure OpSpec for the command to execute.
#[derive(Debug)]
pub struct ShSyncCmdEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for ShSyncCmdEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShSyncCmdData<'op, Id>;
    type Error = ShSyncCmdError;
    type StateDiff = ShSyncCmdStateDiff;
    type StateLogical = ShSyncCmdSyncStatus;
    type StatePhysical = ShSyncCmdExecutionRecord;

    async fn check(
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _file_state_current: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &ShSyncCmdSyncStatus,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<OpCheckStatus, ShSyncCmdError> {
        todo!();
    }

    async fn exec_dry(
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &ShSyncCmdSyncStatus,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<ShSyncCmdExecutionRecord, ShSyncCmdError> {
        todo!()
    }

    async fn exec(
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_desired: &ShSyncCmdSyncStatus,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<ShSyncCmdExecutionRecord, ShSyncCmdError> {
        todo!();
    }
}
