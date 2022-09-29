use std::marker::PhantomData;

use crate::{
    resources::ts::WithStates,
    states::{ts::Cleaned, States, StatesCurrent},
    Resources,
};

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

/// `Resources` is not used at runtime, but is present to signal this type
/// should only be constructed by `CleanCmd`.
impl From<(StatesCurrent, &Resources<WithStates>)> for StatesCleaned {
    fn from((states, _resources): (StatesCurrent, &Resources<WithStates>)) -> Self {
        Self(states.into_inner(), PhantomData)
    }
}
