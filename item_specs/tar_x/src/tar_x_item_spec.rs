use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, state::Nothing, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    TarXCleanOpSpec, TarXEnsureOpSpec, TarXError, TarXState, TarXStateCurrentFnSpec,
    TarXStateDesiredFnSpec, TarXStateDiff, TarXStateDiffFnSpec,
};

/// Item spec for extracting a tar file.
///
/// The `Id` type parameter is needed for each tar extraction params to be a
/// distinct type.
///
/// The following use cases are intended to be supported:
///
/// * A pristine directory with only the tar's contents and nothing else (in
///   progress).
/// * Extraction of a tar over an existing directory (not yet implemented).
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different tar extraction
///   parameters from each other.
#[derive(Debug)]
pub struct TarXItemSpec<Id> {
    /// ID of the item spec to extract the tar.
    item_spec_id: ItemSpecId,
    /// Marker for unique tar extraction parameters type.
    marker: PhantomData<Id>,
}

impl<Id> TarXItemSpec<Id> {
    /// Returns a new `TarXItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for TarXItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type CleanOpSpec = TarXCleanOpSpec<Id>;
    type EnsureOpSpec = TarXEnsureOpSpec<Id>;
    type Error = TarXError;
    type StateCurrentFnSpec = TarXStateCurrentFnSpec<Id>;
    type StateDesiredFnSpec = TarXStateDesiredFnSpec<Id>;
    type StateDiff = TarXStateDiff;
    type StateDiffFnSpec = TarXStateDiffFnSpec;
    type StateLogical = TarXState;
    type StatePhysical = Nothing;

    fn id(&self) -> ItemSpecId {
        self.item_spec_id.clone()
    }

    async fn setup(&self, _resources: &mut Resources<Empty>) -> Result<(), TarXError> {
        Ok(())
    }
}
