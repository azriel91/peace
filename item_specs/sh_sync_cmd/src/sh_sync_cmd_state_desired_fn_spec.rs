use std::marker::PhantomData;

use peace::cfg::{async_trait, OpCtx, State, TryFnSpec};

use crate::{ShSyncCmdData, ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdSyncStatus};

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
    type Output = State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>;

    async fn try_exec(
        op_ctx: OpCtx<'_>,
        sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
    ) -> Result<Option<Self::Output>, ShSyncCmdError> {
        Self::exec(op_ctx, sh_sync_cmd_data).await.map(Some)
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
    ) -> Result<Self::Output, ShSyncCmdError> {
        todo!()
    }
}
