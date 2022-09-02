use std::ops::Deref;

use peace_core::ItemSpecId;
use serde::Serialize;
use type_reg::untagged::TypeMap;

use crate::{resources_type_state::WithStateDiffs, Resources, StatesCurrent};

/// Ensured `State`s for all `ItemSpec`s. `TypeMap<ItemSpecId>` newtype.
///
/// These are the `State`s collected after `EnsureOpSpec::exec` has been run.
///
/// # Implementors
///
/// You may reference [`StatesEnsured`] after `EnsureCmd::exec` has been run.
///
/// [`Data`]: peace_data::Data
#[derive(Debug, Default, Serialize)]
pub struct StatesEnsured(TypeMap<ItemSpecId>);

impl StatesEnsured {
    /// Returns a new `StatesEnsured` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `StatesEnsured` map with the specified capacity.
    ///
    /// The `StatesEnsured` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<ItemSpecId> {
        self.0
    }
}

impl Deref for StatesEnsured {
    type Target = TypeMap<ItemSpecId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TypeMap<ItemSpecId>> for StatesEnsured {
    fn from(type_map: TypeMap<ItemSpecId>) -> Self {
        Self(type_map)
    }
}

/// `Resources` is not used at runtime, but is present to signal this type
/// should only be constructed by `EnsureCmd`.
impl From<(StatesCurrent, &Resources<WithStateDiffs>)> for StatesEnsured {
    fn from((states, _resources): (StatesCurrent, &Resources<WithStateDiffs>)) -> Self {
        Self(states.into_inner())
    }
}
