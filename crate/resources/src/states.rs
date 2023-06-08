//! Resources that track current and goal states, and state diffs.

pub use self::{
    state_diffs::StateDiffs, states_cleaned::StatesCleaned, states_cleaned_dry::StatesCleanedDry,
    states_current::StatesCurrent, states_current_stored::StatesCurrentStored,
    states_ensured::StatesEnsured, states_ensured_dry::StatesEnsuredDry, states_goal::StatesGoal,
    states_goal_stored::StatesGoalStored,
};

pub mod ts;

use std::{marker::PhantomData, ops::Deref};

use peace_core::ItemId;
use peace_fmt::{Presentable, Presenter};
use serde::Serialize;
use type_reg::untagged::{BoxDtDisplay, TypeMap};

use crate::internal::StatesMut;

mod state_diffs;
mod states_cleaned;
mod states_cleaned_dry;
mod states_current;
mod states_current_stored;
mod states_ensured;
mod states_ensured_dry;
mod states_goal;
mod states_goal_stored;

/// Current `State`s for all `Item`s. `TypeMap<ItemId>` newtype.
///
/// Conceptually you can think of this as a `Map<ItemId,
/// Item::State<..>>`.
///
/// # Type Parameters
///
/// * `TS`: Type state to distinguish the purpose of the `States` map.
#[derive(Debug, Serialize)]
#[serde(transparent)] // Needed to serialize as a map instead of a list.
pub struct States<TS>(
    pub(crate) TypeMap<ItemId, BoxDtDisplay>,
    pub(crate) PhantomData<TS>,
);

impl<TS> States<TS> {
    /// Returns a new `States` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `States` map with the specified capacity.
    ///
    /// The `States` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity_typed(capacity), PhantomData)
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<ItemId, BoxDtDisplay> {
        self.0
    }
}

impl<TS> Clone for States<TS> {
    fn clone(&self) -> Self {
        let mut clone = Self(TypeMap::with_capacity_typed(self.0.len()), PhantomData);
        clone.0.extend(
            self.0
                .iter()
                .map(|(item_id, state)| (item_id.clone(), state.clone())),
        );

        clone
    }
}

impl<TS> Default for States<TS> {
    fn default() -> Self {
        Self(TypeMap::default(), PhantomData)
    }
}

impl<TS> Deref for States<TS> {
    type Target = TypeMap<ItemId, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<TS> From<TypeMap<ItemId, BoxDtDisplay>> for States<TS> {
    fn from(type_map: TypeMap<ItemId, BoxDtDisplay>) -> Self {
        Self(type_map, PhantomData)
    }
}

impl<TS> From<StatesMut<TS>> for States<TS> {
    fn from(states_mut: StatesMut<TS>) -> Self {
        Self(states_mut.into_inner(), PhantomData)
    }
}

#[peace_fmt::async_trait(?Send)]
impl<TS> Presentable for States<TS> {
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter
            .list_numbered_with(self.iter(), |(item_id, state)| {
                (item_id, format!(": {state}"))
            })
            .await
    }
}
