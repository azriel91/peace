use peace::cfg::{async_trait, State, StateDiffFnSpec};

use crate::{ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdStateDiff, ShSyncCmdSyncStatus};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct ShSyncCmdStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for ShSyncCmdStateDiffFnSpec {
    type Data<'op> = &'op ();
    type Error = ShSyncCmdError;
    type StateDiff = ShSyncCmdStateDiff;
    type StateLogical = ShSyncCmdSyncStatus;
    type StatePhysical = ShSyncCmdExecutionRecord;

    async fn exec(
        _: &(),
        _state_current: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _state_desired: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
    ) -> Result<Self::StateDiff, ShSyncCmdError> {
        todo!()
    }
}
