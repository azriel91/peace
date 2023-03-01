use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    BlankCleanOpSpec, BlankEnsureOpSpec, BlankError, BlankState, BlankStateCurrentFnSpec,
    BlankStateDesiredFnSpec, BlankStateDiff, BlankStateDiffFnSpec,
};

/// Item spec for copying a number.
///
/// The `Id` type parameter is needed for each blank params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different blank parameters
///   from each other.
#[derive(Debug)]
pub struct BlankItemSpec<Id> {
    /// ID of the blank item spec.
    item_spec_id: ItemSpecId,
    /// Marker for unique blank parameters type.
    marker: PhantomData<Id>,
}

impl<Id> BlankItemSpec<Id> {
    /// Returns a new `BlankItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for BlankItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for BlankItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type CleanOpSpec = BlankCleanOpSpec<Id>;
    type EnsureOpSpec = BlankEnsureOpSpec<Id>;
    type Error = BlankError;
    type State = BlankState;
    type StateCurrentFnSpec = BlankStateCurrentFnSpec<Id>;
    type StateDesiredFnSpec = BlankStateDesiredFnSpec<Id>;
    type StateDiff = BlankStateDiff;
    type StateDiffFnSpec = BlankStateDiffFnSpec;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, _resources: &mut Resources<Empty>) -> Result<(), BlankError> {
        Ok(())
    }
}
