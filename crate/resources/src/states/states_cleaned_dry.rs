use std::marker::PhantomData;

use crate::{
    resources::ts::WithStatesCurrent,
    states::{ts::CleanedDry, States, StatesCurrent},
    Resources,
};

/// Dry-run ensured `State`s for all `ItemSpec`s.
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

/// `Resources` is not used, but is present to signal this type should only be
/// constructed by `CleanCmd`.
impl From<(StatesCurrent, &Resources<WithStatesCurrent>)> for StatesCleanedDry {
    fn from((states, _resources): (StatesCurrent, &Resources<WithStatesCurrent>)) -> Self {
        Self(states.into_inner(), PhantomData)
    }
}
