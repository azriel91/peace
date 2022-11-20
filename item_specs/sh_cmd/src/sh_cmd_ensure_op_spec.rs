use std::marker::PhantomData;

use peace::cfg::state::Nothing;

use peace::cfg::{async_trait, EnsureOpSpec, OpCheckStatus, State};

use crate::{ShCmdData, ShCmdError, ShCmdState, ShCmdStateDiff};

/// Ensure OpSpec for the command to execute.
#[derive(Debug)]
pub struct ShCmdEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for ShCmdEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>
        where Self: 'op;
    type Error = ShCmdError;
    type StateDiff = ShCmdStateDiff;
    type StateLogical = ShCmdState;
    type StatePhysical = Nothing;

    async fn check(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _file_state_current: &State<ShCmdState, Nothing>,
        _file_state_desired: &ShCmdState,
        _diff: &ShCmdStateDiff,
    ) -> Result<OpCheckStatus, ShCmdError> {
        todo!();
    }

    async fn exec_dry(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdState, Nothing>,
        _file_state_desired: &ShCmdState,
        _diff: &ShCmdStateDiff,
    ) -> Result<Nothing, ShCmdError> {
        Ok(Nothing)
    }

    async fn exec(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdState, Nothing>,
        _file_state_desired: &ShCmdState,
        _diff: &ShCmdStateDiff,
    ) -> Result<Nothing, ShCmdError> {
        todo!();
    }
}
