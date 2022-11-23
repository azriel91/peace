use peace::cfg::{async_trait, State, StateDiffFnSpec};

use crate::{ShCmdError, ShCmdExecutionRecord, ShCmdState, ShCmdStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct ShCmdStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for ShCmdStateDiffFnSpec {
    type Data<'op> = &'op ();
    type Error = ShCmdError;
    type StateDiff = ShCmdStateDiff;
    type StateLogical = ShCmdState;
    type StatePhysical = ShCmdExecutionRecord;

    async fn exec(
        _: &(),
        _state_current: &State<ShCmdState, ShCmdExecutionRecord>,
        _state_desired: &ShCmdState,
    ) -> Result<Self::StateDiff, ShCmdError> {
        todo!()
    }
}
