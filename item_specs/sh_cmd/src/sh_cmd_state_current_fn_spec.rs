use std::marker::PhantomData;

use peace::cfg::{async_trait, FnSpec, State};

use crate::{ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdState};

/// Status `FnSpec` for the command to execute.
#[derive(Debug)]
pub struct ShCmdStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> FnSpec for ShCmdStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>;
    type Error = ShCmdError;
    type Output = State<ShCmdState, ShCmdExecutionRecord>;

    async fn exec(_sh_cmd_data: ShCmdData<'_, Id>) -> Result<Self::Output, ShCmdError> {
        todo!()
    }
}
