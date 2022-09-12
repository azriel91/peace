//! Resources that track current and desired states, and state diffs.

pub use self::{
    state_diffs::StateDiffs, states_current::StatesCurrent, states_desired::StatesDesired,
    states_ensured::StatesEnsured, states_ensured_dry::StatesEnsuredDry,
};

pub mod ts;

use std::{marker::PhantomData, ops::Deref};

use peace_core::ItemSpecId;
use serde::Serialize;
use type_reg::untagged::TypeMap;

use crate::internal::StatesMut;

mod state_diffs;
mod states_current;
mod states_desired;
mod states_ensured;
mod states_ensured_dry;

/// Current `State`s for all `ItemSpec`s. `TypeMap<ItemSpecId>` newtype.
///
/// Conceptually you can think of this as a `Map<ItemSpecId,
/// ItemSpec::State<..>>`.
///
/// # Type Parameters
///
/// * `TS`: Type state to distinguish the purpose of the `States` map.
#[derive(Debug, Serialize)]
#[serde(transparent)] // Needed to serialize as a map instead of a list.
pub struct States<TS>(pub(crate) TypeMap<ItemSpecId>, pub(crate) PhantomData<TS>);

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
        Self(TypeMap::with_capacity(capacity), PhantomData)
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<ItemSpecId> {
        self.0
    }
}

impl<TS> Default for States<TS> {
    fn default() -> Self {
        Self(TypeMap::default(), PhantomData)
    }
}

impl<TS> Deref for States<TS> {
    type Target = TypeMap<ItemSpecId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<TS> From<TypeMap<ItemSpecId>> for States<TS> {
    fn from(type_map: TypeMap<ItemSpecId>) -> Self {
        Self(type_map, PhantomData)
    }
}

impl<TS> From<StatesMut<TS>> for States<TS> {
    fn from(states_mut: StatesMut<TS>) -> Self {
        Self(states_mut.into_inner(), PhantomData)
    }
}
