use peace::cfg::{async_trait, State, StateDiffFnSpec};

use crate::{ShCmdError, ShCmdExecutionRecord, ShCmdStateDiff, ShCmdSyncStatus};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct ShCmdStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for ShCmdStateDiffFnSpec {
    type Data<'op> = &'op ();
    type Error = ShCmdError;
    type StateDiff = ShCmdStateDiff;
    type StateLogical = ShCmdSyncStatus;
    type StatePhysical = ShCmdExecutionRecord;

    async fn exec(
        _: &(),
        _state_current: &State<ShCmdSyncStatus, ShCmdExecutionRecord>,
        _state_desired: &ShCmdSyncStatus,
    ) -> Result<Self::StateDiff, ShCmdError> {
        todo!()
    }
}
