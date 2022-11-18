use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, state::Nothing, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    ShCmdCleanOpSpec, ShCmdEnsureOpSpec, ShCmdError, ShCmdState, ShCmdStateCurrentFnSpec,
    ShCmdStateDesiredFnSpec, ShCmdStateDiff, ShCmdStateDiffFnSpec,
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
pub struct ShCmdItemSpec<Id> {
    /// ID to easily tell what the item spec command is for.
    item_spec_id: ItemSpecId,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> ShCmdItemSpec<Id> {
    /// Returns a new `ShCmdItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for ShCmdItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type CleanOpSpec = ShCmdCleanOpSpec<Id>;
    type EnsureOpSpec = ShCmdEnsureOpSpec<Id>;
    type Error = ShCmdError;
    type StateCurrentFnSpec = ShCmdStateCurrentFnSpec<Id>;
    type StateDesiredFnSpec = ShCmdStateDesiredFnSpec<Id>;
    type StateDiff = ShCmdStateDiff;
    type StateDiffFnSpec = ShCmdStateDiffFnSpec;
    type StateLogical = ShCmdState;
    type StatePhysical = Nothing;

    fn id(&self) -> ItemSpecId {
        self.item_spec_id.clone()
    }

    async fn setup(&self, _resources: &mut Resources<Empty>) -> Result<(), ShCmdError> {
        Ok(())
    }
}
