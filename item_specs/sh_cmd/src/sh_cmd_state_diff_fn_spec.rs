#[nougat::gat(Data)]
use peace::cfg::StateDiffFnSpec;
use peace::cfg::{async_trait, nougat, state::Nothing, State};

use crate::{ShCmdError, ShCmdState, ShCmdStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct ShCmdStateDiffFnSpec;

#[async_trait(?Send)]
#[nougat::gat]
impl StateDiffFnSpec for ShCmdStateDiffFnSpec {
    type Data<'op> = &'op()
        where Self: 'op;
    type Error = ShCmdError;
    type StateDiff = ShCmdStateDiff;
    type StateLogical = ShCmdState;
    type StatePhysical = Nothing;

    async fn exec(
        _: &(),
        _state_current: &State<ShCmdState, Nothing>,
        _state_desired: &ShCmdState,
    ) -> Result<Self::StateDiff, ShCmdError> {
        todo!()
    }
}
