use std::marker::PhantomData;

use peace_core::ItemId;

use crate::states::{ts::CleanedDry, States, StatesCurrent};

/// Dry-run ensured `State`s for all `Item`s.
///
/// These are the `State`s collected after `CleanOpSpec::exec_dry` has been
/// run.
///
/// # Implementors
///
/// You may reference [`StatesCleanedDry<ItemIdT>`] after `CleanCmd::exec_dry`
/// has been run.
///
/// [`Data`]: peace_data::Data
pub type StatesCleanedDry<ItemIdT> = States<ItemIdT, CleanedDry>;

impl<ItemIdT> From<StatesCurrent<ItemIdT>> for StatesCleanedDry<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn from(states: StatesCurrent<ItemIdT>) -> Self {
        Self(states.into_inner(), PhantomData)
    }
}
