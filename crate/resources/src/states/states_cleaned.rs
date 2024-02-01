use std::marker::PhantomData;

use peace_core::ItemId;

use crate::states::{ts::Cleaned, States, StatesCurrent};

/// Cleaned `State`s for all `Item`s. `TypeMap<ItemId>` newtype.
///
/// These are the `State`s collected after `CleanOpSpec::exec` has been run.
///
/// **Note:** Not to be confused with [`StatesClean`].
///
/// [`StatesClean`]: crate::states::StatesClean
///
/// # Implementors
///
/// You may reference [`StatesCleaned`] after `CleanCmd::exec` has been run,
/// unless it is the `ExecutionOutcome`.
pub type StatesCleaned<ItemIdT> = States<ItemIdT, Cleaned>;

impl<ItemIdT> From<StatesCurrent<ItemIdT>> for StatesCleaned<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn from(states: StatesCurrent<ItemIdT>) -> Self {
        Self(states.into_inner(), PhantomData)
    }
}
