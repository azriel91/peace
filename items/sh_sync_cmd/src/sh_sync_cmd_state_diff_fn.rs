use std::marker::PhantomData;

use peace::cfg::State;

use crate::{ShSyncCmdError, ShSyncCmdExecutionRecord, ShSyncCmdStateDiff, ShSyncCmdSyncStatus};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct ShSyncCmdStateDiffFn<Id>(PhantomData<Id>);

impl<Id> ShSyncCmdStateDiffFn<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn state_diff(
        _state_current: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
        _state_goal: &State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>,
    ) -> Result<ShSyncCmdStateDiff, ShSyncCmdError> {
        todo!()
    }
}
