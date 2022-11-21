use std::marker::PhantomData;

use peace::cfg::{async_trait, CleanOpSpec, OpCheckStatus, State};

use crate::{ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdSyncStatus};

/// `CleanOpSpec` for the command to execute.
#[derive(Debug, Default)]
pub struct ShCmdCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> CleanOpSpec for ShCmdCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>;
    type Error = ShCmdError;
    type StateLogical = ShCmdSyncStatus;
    type StatePhysical = ShCmdExecutionRecord;

    async fn check(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdSyncStatus, ShCmdExecutionRecord>,
    ) -> Result<OpCheckStatus, ShCmdError> {
        todo!()
    }

    async fn exec_dry(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdSyncStatus, ShCmdExecutionRecord>,
    ) -> Result<(), ShCmdError> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdSyncStatus, ShCmdExecutionRecord>,
    ) -> Result<(), ShCmdError> {
        todo!()
    }

    #[cfg(target_arch = "wasm32")]
    async fn exec(
        sh_cmd_data: ShCmdData<'_, Id>,
        State {
            logical: file_state,
            ..
        }: &State<ShCmdSyncStatus, ShCmdExecutionRecord>,
    ) -> Result<(), ShCmdError> {
        todo!()
    }
}
