use std::marker::PhantomData;

#[nougat::gat(Data)]
use peace::cfg::CleanOpSpec;
use peace::cfg::{async_trait, nougat, state::Nothing, OpCheckStatus, State};

use crate::{ShCmdData, ShCmdError, ShCmdState};

/// `CleanOpSpec` for the command to execute.
#[derive(Debug, Default)]
pub struct ShCmdCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
#[nougat::gat]
impl<Id> CleanOpSpec for ShCmdCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>
        where Self: 'op;
    type Error = ShCmdError;
    type StateLogical = ShCmdState;
    type StatePhysical = Nothing;

    async fn check(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdState, Nothing>,
    ) -> Result<OpCheckStatus, ShCmdError> {
        todo!()
    }

    async fn exec_dry(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdState, Nothing>,
    ) -> Result<(), ShCmdError> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdState, Nothing>,
    ) -> Result<(), ShCmdError> {
        todo!()
    }

    #[cfg(target_arch = "wasm32")]
    async fn exec(
        sh_cmd_data: ShCmdData<'_, Id>,
        State {
            logical: file_state,
            ..
        }: &State<ShCmdState, Nothing>,
    ) -> Result<(), ShCmdError> {
        todo!()
    }
}
