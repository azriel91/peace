use std::marker::PhantomData;

use peace::cfg::{async_trait, FnSpec};

use crate::{ShSyncCmdData, ShSyncCmdError, ShSyncCmdSyncStatus};

/// Status desired `FnSpec` for the command to execute.
#[derive(Debug)]
pub struct ShSyncCmdStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> FnSpec for ShSyncCmdStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShSyncCmdData<'op, Id>;
    type Error = ShSyncCmdError;
    type Output = Option<ShSyncCmdSyncStatus>;

    async fn exec(
        _sh_sync_cmd_data: ShSyncCmdData<'_, Id>,
    ) -> Result<Self::Output, ShSyncCmdError> {
        todo!()
    }
}
