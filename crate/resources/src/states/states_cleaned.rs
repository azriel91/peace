use std::marker::PhantomData;

use crate::states::{ts::Cleaned, States, StatesCurrent};

/// Cleaned `State`s for all `ItemSpec`s. `TypeMap<ItemSpecId>` newtype.
///
/// These are the `State`s collected after `CleanOpSpec::exec` has been run.
///
/// # Implementors
///
/// You may reference [`StatesCleaned`] after `CleanCmd::exec` has been run.
///
/// [`Data`]: peace_data::Data
pub type StatesCleaned = States<Cleaned>;

impl From<StatesCurrent> for StatesCleaned {
    fn from(states: StatesCurrent) -> Self {
        Self(states.into_inner(), PhantomData)
    }
}
