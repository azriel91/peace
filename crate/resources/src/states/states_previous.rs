use std::marker::PhantomData;

use peace_core::ItemId;

use crate::states::{ts::Previous, States, StatesCurrent};

/// Previous `State`s for all `Item`s.
///
/// This is present when an `ApplyCmd` (`EnsureCmd` or `CleanCmd`) is run,
/// whereby the current states have changed to the newly ensured states.
pub type StatesPrevious<ItemIdT> = States<ItemIdT, Previous>;

impl<ItemIdT> From<StatesCurrent<ItemIdT>> for StatesPrevious<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn from(states_current: StatesCurrent<ItemIdT>) -> Self {
        Self(states_current.into_inner(), PhantomData)
    }
}
