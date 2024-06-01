use std::marker::PhantomData;

use crate::states::{ts::Cleaned, States, StatesCurrent};

/// Cleaned `State`s for all `Step`s. `TypeMap<StepId>` newtype.
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
pub type StatesCleaned = States<Cleaned>;

impl From<StatesCurrent> for StatesCleaned {
    fn from(states: StatesCurrent) -> Self {
        Self(states.into_inner(), PhantomData)
    }
}
