use std::marker::PhantomData;

#[nougat::gat(Data)]
use peace::cfg::FnSpec;
use peace::cfg::{async_trait, nougat};

use crate::{ShCmdData, ShCmdError, ShCmdState};

/// Status desired `FnSpec` for the command to execute.
#[derive(Debug)]
pub struct ShCmdStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
#[nougat::gat]
impl<Id> FnSpec for ShCmdStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>
        where Self: 'op;
    type Error = ShCmdError;
    type Output = ShCmdState;

    async fn exec(_sh_cmd_data: ShCmdData<'_, Id>) -> Result<Self::Output, ShCmdError> {
        todo!()
    }
}
