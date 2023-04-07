use std::marker::PhantomData;

use crate::states::{ts::EnsuredDry, States, StatesCurrent};

/// Dry-run ensured `State`s for all `ItemSpec`s.
///
/// These are the `State`s collected after `ApplyFns::exec_dry` has been
/// run.
///
/// # Implementors
///
/// You may reference [`StatesEnsuredDry`] after `EnsureCmd::exec_dry` has been
/// run.
///
/// [`Data`]: peace_data::Data
pub type StatesEnsuredDry = States<EnsuredDry>;

impl From<StatesCurrent> for StatesEnsuredDry {
    fn from(states_current: StatesCurrent) -> Self {
        Self(states_current.into_inner(), PhantomData)
    }
}
