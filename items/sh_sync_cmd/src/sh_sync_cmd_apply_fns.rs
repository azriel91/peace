use std::marker::PhantomData;

use peace::cfg::{ApplyCheck, FnCtx, State};

use crate::{
    ShSyncCmdData, ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdParams, ShSyncCmdStateDiff,
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
        _params: &ShSyncCmdParams<Id>,
        _data: ShSyncCmdData<'_, Id>,
        _file_state_current: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_goal: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<ApplyCheck, ShSyncCmdError> {
        todo!();
    }

    pub async fn apply_dry(
        _fn_ctx: FnCtx<'_>,
        _params: &ShSyncCmdParams<Id>,
        _data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_goal: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>, ShSyncCmdError> {
        todo!()
    }

    pub async fn apply(
        _fn_ctx: FnCtx<'_>,
        _params: &ShSyncCmdParams<Id>,
        _data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _file_state_goal: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _diff: &ShSyncCmdStateDiff,
    ) -> Result<State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>, ShSyncCmdError> {
        todo!();
    }
}
