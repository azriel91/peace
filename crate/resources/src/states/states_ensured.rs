use std::marker::PhantomData;

use crate::states::{ts::Ensured, States, StatesCurrent};

/// Ensured `State`s for all `ItemSpec`s. `TypeMap<ItemSpecId>` newtype.
///
/// These are the `State`s collected after `ApplyFns::exec` has been run.
///
/// # Implementors
///
/// You may reference [`StatesEnsured`] after `EnsureCmd::exec` has been run.
///
/// [`Data`]: peace_data::Data
pub type StatesEnsured = States<Ensured>;

impl From<StatesCurrent> for StatesEnsured {
    fn from(states_current: StatesCurrent) -> Self {
        Self(states_current.into_inner(), PhantomData)
    }
}
