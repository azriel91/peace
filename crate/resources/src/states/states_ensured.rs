use std::marker::PhantomData;

use crate::{
    resources::ts::WithStateCurrentDiffs,
    states::{ts::Ensured, States, StatesCurrent},
    Resources,
};

/// Ensured `State`s for all `ItemSpec`s. `TypeMap<ItemSpecId>` newtype.
///
/// These are the `State`s collected after `EnsureOpSpec::exec` has been run.
///
/// # Implementors
///
/// You may reference [`StatesEnsured`] after `EnsureCmd::exec` has been run.
///
/// [`Data`]: peace_data::Data
pub type StatesEnsured = States<Ensured>;

/// `Resources` is not used, but is present to signal this type should only be
/// constructed by `EnsureCmd`.
impl From<(StatesCurrent, &Resources<WithStateCurrentDiffs>)> for StatesEnsured {
    fn from((states, _resources): (StatesCurrent, &Resources<WithStateCurrentDiffs>)) -> Self {
        Self(states.into_inner(), PhantomData)
    }
}
