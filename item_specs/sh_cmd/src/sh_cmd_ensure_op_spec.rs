use std::marker::PhantomData;

use peace::cfg::{async_trait, EnsureOpSpec, OpCheckStatus, State};

use crate::{ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdState, ShCmdStateDiff};

/// Ensure OpSpec for the command to execute.
#[derive(Debug)]
pub struct ShCmdEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for ShCmdEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>;
    type Error = ShCmdError;
    type StateDiff = ShCmdStateDiff;
    type StateLogical = ShCmdState;
    type StatePhysical = ShCmdExecutionRecord;

    async fn check(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _file_state_current: &State<ShCmdState, ShCmdExecutionRecord>,
        _file_state_desired: &ShCmdState,
        _diff: &ShCmdStateDiff,
    ) -> Result<OpCheckStatus, ShCmdError> {
        todo!();
    }

    async fn exec_dry(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdState, ShCmdExecutionRecord>,
        _file_state_desired: &ShCmdState,
        _diff: &ShCmdStateDiff,
    ) -> Result<ShCmdExecutionRecord, ShCmdError> {
        todo!()
    }

    async fn exec(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdState, ShCmdExecutionRecord>,
        _file_state_desired: &ShCmdState,
        _diff: &ShCmdStateDiff,
    ) -> Result<ShCmdExecutionRecord, ShCmdError> {
        todo!();
    }
}
