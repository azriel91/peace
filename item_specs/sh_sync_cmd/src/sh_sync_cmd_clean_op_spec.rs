use std::marker::PhantomData;

use peace::cfg::{async_trait, CleanOpSpec, OpCheckStatus, State};

use crate::{ShSyncCmdData, ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdSyncStatus};

/// `CleanOpSpec` for the command to execute.
#[derive(Debug, Default)]
pub struct ShSyncCmdCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> CleanOpSpec for ShSyncCmdCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShSyncCmdData<'op, Id>;
    type Error = ShSyncCmdError;
    type StateLogical = ShSyncCmdSyncStatus;
    type StatePhysical = ShSyncCmdExecutionRecord;

    async fn check(
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
    ) -> Result<OpCheckStatus, ShSyncCmdError> {
        todo!()
    }

    async fn exec_dry(
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
    ) -> Result<(), ShSyncCmdError> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        _state: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
    ) -> Result<(), ShSyncCmdError> {
        todo!()
    }

    #[cfg(target_arch = "wasm32")]
    async fn exec(
        sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
        State {
            logical: file_state,
            ..
        }: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
    ) -> Result<(), ShSyncCmdError> {
        todo!()
    }
}
