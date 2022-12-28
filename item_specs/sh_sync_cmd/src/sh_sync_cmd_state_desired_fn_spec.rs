use std::marker::PhantomData;

use peace::cfg::{async_trait, TryFnSpec};

use crate::{ShSyncCmdData, ShSyncCmdError, ShSyncCmdSyncStatus};

/// Reads the desired state of the command to execute.
#[derive(Debug)]
pub struct ShSyncCmdStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for ShSyncCmdStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShSyncCmdData<'op, Id>;
    type Error = ShSyncCmdError;
    type Output = ShSyncCmdSyncStatus;

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
