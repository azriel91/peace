use std::marker::PhantomData;

use peace::cfg::{async_trait, State, TryFnSpec};

use crate::{ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdState};

/// Reads the current state of the command to execute.
#[derive(Debug)]
pub struct ShCmdStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for ShCmdStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>;
    type Error = ShCmdError;
    type Output = State<ShCmdState<Id>, ShCmdExecutionRecord>;

    async fn try_exec(sh_cmd_data: ShCmdData<'_, Id>) -> Result<Option<Self::Output>, ShCmdError> {
        Self::exec(sh_cmd_data).await.map(Some)
    }

    async fn exec(sh_cmd_data: ShCmdData<'_, Id>) -> Result<Self::Output, ShCmdError> {
        let state_current_sh_cmd = sh_cmd_data.sh_cmd_params().state_current_sh_cmd();
        ShCmdExecutor::exec(state_current_sh_cmd).await
    }
}
