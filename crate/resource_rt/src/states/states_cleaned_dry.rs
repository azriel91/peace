use std::marker::PhantomData;

use crate::states::{ts::CleanedDry, States, StatesCurrent};

/// Dry-run ensured `State`s for all `Item`s.
///
/// These are the `State`s collected after `CleanOpSpec::exec_dry` has been
/// run.
///
/// # Implementors
///
/// You may reference [`StatesCleanedDry`] after `CleanCmd::exec_dry` has been
/// run.
///
/// [`Data`]: peace_data::Data
pub type StatesCleanedDry = States<CleanedDry>;

impl From<StatesCurrent> for StatesCleanedDry {
    fn from(states: StatesCurrent) -> Self {
        Self(states.into_inner(), PhantomData)
    }
}
