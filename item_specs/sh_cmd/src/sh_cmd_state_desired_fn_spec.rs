use std::marker::PhantomData;

use peace::cfg::{async_trait, OpCtx, State, TryFnSpec};

use crate::{ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdState};

/// Reads the desired state of the command to execute.
#[derive(Debug)]
pub struct ShCmdStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for ShCmdStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>;
    type Error = ShCmdError;
    type Output = State<ShCmdState<Id>, ShCmdExecutionRecord>;

    async fn try_exec(
        op_ctx: OpCtx<'_>,
        sh_cmd_data: ShCmdData<'_, Id>,
    ) -> Result<Option<Self::Output>, ShCmdError> {
        Self::exec(op_ctx, sh_cmd_data).await.map(Some)
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        sh_cmd_data: ShCmdData<'_, Id>,
    ) -> Result<Self::Output, ShCmdError> {
        let state_desired_sh_cmd = sh_cmd_data.sh_cmd_params().state_desired_sh_cmd();
        // Maybe we should support reading different exit statuses for an `Ok(None)`
        // value.
        ShCmdExecutor::exec(state_desired_sh_cmd).await
    }
}
