use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId, State},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    ShSyncCmdApplyOpSpec, ShSyncCmdCleanOpSpec, ShSyncCmdData, ShSyncCmdError,
    ShSyncCmdExecutionRecord, ShSyncCmdStateCurrentFnSpec, ShSyncCmdStateDesiredFnSpec,
    ShSyncCmdStateDiff, ShSyncCmdStateDiffFnSpec, ShSyncCmdSyncStatus,
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

impl<Id> Clone for ShSyncCmdItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
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
    type ApplyOpSpec = ShSyncCmdApplyOpSpec<Id>;
    type CleanOpSpec = ShSyncCmdCleanOpSpec<Id>;
    type Data<'op> = ShSyncCmdData<'op, Id>;
    type Error = ShSyncCmdError;
    type State = State<ShSyncCmdSyncStatus, ShSyncCmdExecutionRecord>;
    type StateCurrentFnSpec = ShSyncCmdStateCurrentFnSpec<Id>;
    type StateDesiredFnSpec = ShSyncCmdStateDesiredFnSpec<Id>;
    type StateDiff = ShSyncCmdStateDiff;
    type StateDiffFnSpec = ShSyncCmdStateDiffFnSpec;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, _resources: &mut Resources<Empty>) -> Result<(), ShSyncCmdError> {
        Ok(())
    }

    async fn state_clean(_: Self::Data<'_>) -> Result<Self::State, ShSyncCmdError> {
        let state = State::new(
            ShSyncCmdSyncStatus::NotExecuted,
            ShSyncCmdExecutionRecord::None,
        );
        Ok(state)
    }
}
