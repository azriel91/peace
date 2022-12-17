use std::marker::PhantomData;

use crate::{
    resources::ts::WithStatesCurrentDiffs,
    states::{ts::EnsuredDry, States, StatesCurrent},
    Resources,
};

/// Dry-run ensured `State`s for all `ItemSpec`s.
///
/// These are the `State`s collected after `EnsureOpSpec::exec_dry` has been
/// run.
///
/// # Implementors
///
/// You may reference [`StatesEnsuredDry`] after `EnsureCmd::exec_dry` has been
/// run.
///
/// [`Data`]: peace_data::Data
pub type StatesEnsuredDry = States<EnsuredDry>;

/// `Resources` is not used, but is present to signal this type should only be
/// constructed by `EnsureCmd`.
impl From<(StatesCurrent, &Resources<WithStatesCurrentDiffs>)> for StatesEnsuredDry {
    fn from((states, _resources): (StatesCurrent, &Resources<WithStatesCurrentDiffs>)) -> Self {
        Self(states.into_inner(), PhantomData)
    }
}
