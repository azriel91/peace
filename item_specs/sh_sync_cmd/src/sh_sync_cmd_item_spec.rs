use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    ShSyncCmdCleanOpSpec, ShSyncCmdEnsureOpSpec, ShSyncCmdError, ShSyncCmdExecutionRecord,
    ShSyncCmdStateCurrentFnSpec, ShSyncCmdStateDesiredFnSpec, ShSyncCmdStateDiff,
    ShSyncCmdStateDiffFnSpec, ShSyncCmdSyncStatus,
};

/// Item spec for executing a shell command.
///
/// The `Id` type parameter is needed for each command execution params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Debug)]
pub struct ShSyncCmdItemSpec<Id> {
    /// ID to easily tell what the item spec command is for.
    item_spec_id: ItemSpecId,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> ShSyncCmdItemSpec<Id> {
    /// Returns a new `ShSyncCmdItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for ShSyncCmdItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type CleanOpSpec = ShSyncCmdCleanOpSpec<Id>;
    type EnsureOpSpec = ShSyncCmdEnsureOpSpec<Id>;
    type Error = ShSyncCmdError;
    type StateCurrentFnSpec = ShSyncCmdStateCurrentFnSpec<Id>;
    type StateDesiredFnSpec = ShSyncCmdStateDesiredFnSpec<Id>;
    type StateDiff = ShSyncCmdStateDiff;
    type StateDiffFnSpec = ShSyncCmdStateDiffFnSpec;
    type StateLogical = ShSyncCmdSyncStatus;
    type StatePhysical = ShSyncCmdExecutionRecord;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, _resources: &mut Resources<Empty>) -> Result<(), ShSyncCmdError> {
        Ok(())
    }
}
