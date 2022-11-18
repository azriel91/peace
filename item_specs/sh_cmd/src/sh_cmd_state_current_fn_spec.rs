use std::marker::PhantomData;

use peace::cfg::FnSpec;
use peace::cfg::{async_trait, state::Nothing, State};

use crate::{ShCmdData, ShCmdError, ShCmdState};

/// Status `FnSpec` for the command to execute.
#[derive(Debug)]
pub struct ShCmdStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> FnSpec for ShCmdStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>
        where Self: 'op;
    type Error = ShCmdError;
    type Output = State<ShCmdState, Nothing>;

    async fn exec(_sh_cmd_data: ShCmdData<'_, Id>) -> Result<Self::Output, ShCmdError> {
        todo!()
    }
}
