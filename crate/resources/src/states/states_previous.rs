use std::marker::PhantomData;

use crate::states::{ts::Previous, States, StatesCurrent};

/// Previous `State`s for all `Step`s.
///
/// This is present when an `ApplyCmd` (`EnsureCmd` or `CleanCmd`) is run,
/// whereby the current states have changed to the newly ensured states.
pub type StatesPrevious = States<Previous>;

impl From<StatesCurrent> for StatesPrevious {
    fn from(states_current: StatesCurrent) -> Self {
        Self(states_current.into_inner(), PhantomData)
    }
}
