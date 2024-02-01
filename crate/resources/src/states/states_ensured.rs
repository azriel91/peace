use std::marker::PhantomData;

use peace_core::ItemId;

use crate::states::{ts::Ensured, States, StatesCurrent};

/// Ensured `State`s for all `Item`s. `TypeMap<ItemId>` newtype.
///
/// These are the `State`s collected after `ApplyFns::exec` has been run.
///
/// # Implementors
///
/// You may reference [`StatesEnsured<ItemIdT>`] after `EnsureCmd::exec` has
/// been run.
///
/// [`Data`]: peace_data::Data
pub type StatesEnsured<ItemIdT> = States<ItemIdT, Ensured>;

impl<ItemIdT> From<StatesCurrent<ItemIdT>> for StatesEnsured<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn from(states_current: StatesCurrent<ItemIdT>) -> Self {
        Self(states_current.into_inner(), PhantomData)
    }
}
