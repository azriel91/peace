use std::marker::PhantomData;

use peace::cfg::{async_trait, FnSpec};

use crate::{ShCmdData, ShCmdError, ShCmdSyncStatus};

/// Status desired `FnSpec` for the command to execute.
#[derive(Debug)]
pub struct ShCmdStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> FnSpec for ShCmdStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>;
    type Error = ShCmdError;
    type Output = ShCmdSyncStatus;

    async fn exec(_sh_cmd_data: ShCmdData<'_, Id>) -> Result<Self::Output, ShCmdError> {
        todo!()
    }
}
