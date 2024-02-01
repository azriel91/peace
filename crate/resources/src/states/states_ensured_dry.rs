use std::marker::PhantomData;

use peace_core::ItemId;

use crate::states::{ts::EnsuredDry, States, StatesCurrent};

/// Dry-run ensured `State`s for all `Item`s.
///
/// These are the `State`s collected after `ApplyFns::exec_dry` has been
/// run.
///
/// # Implementors
///
/// You may reference [`StatesEnsuredDry<ItemIdT>`] after `EnsureCmd::exec_dry`
/// has been run.
///
/// [`Data`]: peace_data::Data
pub type StatesEnsuredDry<ItemIdT> = States<ItemIdT, EnsuredDry>;

impl<ItemIdT> From<StatesCurrent<ItemIdT>> for StatesEnsuredDry<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn from(states_current: StatesCurrent<ItemIdT>) -> Self {
        Self(states_current.into_inner(), PhantomData)
    }
}
