use std::ops::Deref;

use peace_core::FullSpecId;
use serde::Serialize;
use type_reg::untagged::TypeMap;

use crate::StateDiffsMut;

/// Diffs of `StateLogical`s for each `FullSpec`s. `TypeMap<FullSpecId>`
/// newtype.
///
/// # Implementors
///
/// [`StateDiffs`] is a read-only resource, stored in [`Resources`] after
/// `DiffCmd` has been executed.
///
/// [`Resources`]: crate::Resources
#[derive(Debug, Default, Serialize)]
pub struct StateDiffs(TypeMap<FullSpecId>);

impl StateDiffs {
    /// Returns a new `StateDiffs` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `StateDiffs` map with the specified capacity.
    ///
    /// The `StateDiffs` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<FullSpecId> {
        self.0
    }
}

impl Deref for StateDiffs {
    type Target = TypeMap<FullSpecId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TypeMap<FullSpecId>> for StateDiffs {
    fn from(type_map: TypeMap<FullSpecId>) -> Self {
        Self(type_map)
    }
}

impl From<StateDiffsMut> for StateDiffs {
    fn from(states_desired_mut: StateDiffsMut) -> Self {
        Self(states_desired_mut.into_inner())
    }
}
