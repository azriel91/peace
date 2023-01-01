use std::marker::PhantomData;

use peace::cfg::{async_trait, State, TryFnSpec};

use crate::{ShSyncCmdData, ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdSyncStatus};

/// Reads the current state of the command to execute.
#[derive(Debug)]
pub struct ShSyncCmdStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for ShSyncCmdStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShSyncCmdData<'op, Id>;
    type Error = ShSyncCmdError;
    type Output = State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>;

    async fn try_exec(
        sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
    ) -> Result<Option<Self::Output>, ShSyncCmdError> {
        Self::exec(sh_sync_cmd_data).await.map(Some)
    }

    async fn exec(
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
    ) -> Result<Self::Output, ShSyncCmdError> {
        todo!()
    }
}
