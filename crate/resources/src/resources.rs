use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{
    resources_type_state::{
        Empty, Ensured, EnsuredDry, SetUp, WithStateDiffs, WithStates, WithStatesCurrentAndDesired,
        WithStatesDesired,
    },
    StateDiffs, States, StatesDesired, StatesEnsured, StatesEnsuredDry,
};

/// Map of all types at runtime. [`resman::Resources`] newtype.
///
/// This augments the any-map functionality of [`resman::Resources`] with type
/// state, so that it is impossible for developers to pass `Resources` to
/// functions that require particular operations to have executed over the
/// resources beforehand.
///
/// For example, `Resources` must be `setup` before any `FnSpec` or `OpSpec` may
/// execute with it.
///
/// # Type Parameters
///
/// * `TS`: The type state of the `Resources` map.
///
/// [`FullSpecId`]: peace_cfg::FullSpecId
#[derive(Debug)]
pub struct Resources<TS> {
    inner: resman::Resources,
    marker: PhantomData<TS>,
}

impl Resources<Empty> {
    /// Returns a new `Resources`.
    pub fn new() -> Self {
        Self {
            inner: resman::Resources::new(),
            marker: PhantomData,
        }
    }
}

impl<TS> Resources<TS> {
    /// Returns the inner [`resman::Resources`].
    pub fn into_inner(self) -> resman::Resources {
        self.inner
    }
}

impl Default for Resources<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TS> Deref for Resources<TS> {
    type Target = resman::Resources;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<TS> DerefMut for Resources<TS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// For `FullSpecGraph` after resources have been set up.
impl From<Resources<Empty>> for Resources<SetUp> {
    fn from(resources: Resources<Empty>) -> Self {
        Self {
            inner: resources.into_inner(),
            marker: PhantomData,
        }
    }
}

// For `StateCurrentCmd` after `States` have been discovered.
impl From<(Resources<SetUp>, States)> for Resources<WithStates> {
    fn from((mut resources, states): (Resources<SetUp>, States)) -> Self {
        resources.insert(states);

        Self {
            inner: resources.into_inner(),
            marker: PhantomData,
        }
    }
}

// For `StateDesiredCmd` after `StatesDesired` have been discovered.
impl From<(Resources<SetUp>, StatesDesired)> for Resources<WithStatesDesired> {
    fn from((mut resources, states_desired): (Resources<SetUp>, StatesDesired)) -> Self {
        resources.insert(states_desired);

        Self {
            inner: resources.into_inner(),
            marker: PhantomData,
        }
    }
}

impl From<(Resources<SetUp>, States, StatesDesired)> for Resources<WithStatesCurrentAndDesired> {
    fn from(
        (mut resources, states, states_desired): (Resources<SetUp>, States, StatesDesired),
    ) -> Self {
        resources.insert(states);
        resources.insert(states_desired);

        Self {
            inner: resources.into_inner(),
            marker: PhantomData,
        }
    }
}

impl From<(Resources<WithStatesCurrentAndDesired>, StateDiffs)> for Resources<WithStateDiffs> {
    fn from(
        (mut resources, state_diffs): (Resources<WithStatesCurrentAndDesired>, StateDiffs),
    ) -> Self {
        resources.insert(state_diffs);

        Self {
            inner: resources.into_inner(),
            marker: PhantomData,
        }
    }
}

impl From<(Resources<WithStateDiffs>, StatesEnsuredDry)> for Resources<EnsuredDry> {
    fn from(
        (mut resources, states_ensured_dry): (Resources<WithStateDiffs>, StatesEnsuredDry),
    ) -> Self {
        resources.insert(states_ensured_dry);

        Self {
            inner: resources.into_inner(),
            marker: PhantomData,
        }
    }
}

impl From<(Resources<WithStateDiffs>, StatesEnsured)> for Resources<Ensured> {
    fn from((mut resources, states_ensured): (Resources<WithStateDiffs>, StatesEnsured)) -> Self {
        resources.insert(states_ensured);

        Self {
            inner: resources.into_inner(),
            marker: PhantomData,
        }
    }
}
