use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    ShCmdCleanOpSpec, ShCmdEnsureOpSpec, ShCmdError, ShCmdExecutionRecord, ShCmdParams, ShCmdState,
    ShCmdStateCurrentFnSpec, ShCmdStateDesiredFnSpec, ShCmdStateDiff, ShCmdStateDiffFnSpec,
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
    /// Parameters to insert into `resources` in [`ItemSpec::setup`].
    sh_cmd_params: Option<ShCmdParams<Id>>,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> ShCmdItemSpec<Id> {
    /// Returns a new `ShCmdItemSpec`.
    ///
    /// # Parameters
    ///
    /// * `item_spec_id`: ID of this `ShCmdItemSpec`.
    /// * `sh_cmd_params`: Parameters to insert into `Resources`.
    pub fn new(item_spec_id: ItemSpecId, sh_cmd_params: Option<ShCmdParams<Id>>) -> Self {
        Self {
            item_spec_id,
            sh_cmd_params,
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
    type StateDiffFnSpec = ShCmdStateDiffFnSpec<Id>;
    type StateLogical = ShCmdState<Id>;
    type StatePhysical = ShCmdExecutionRecord;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), ShCmdError> {
        if let Some(sh_cmd_params) = self.sh_cmd_params.clone() {
            resources.insert(sh_cmd_params);
        }

        Ok(())
    }
}
