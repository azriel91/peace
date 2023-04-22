use std::ops::{Deref, DerefMut};

use peace_core::ItemSpecId;
use serde::Serialize;
use type_reg::untagged::{BoxDt, TypeMap};

/// Map of item spec ID to its parameters. `TypeMap<ItemSpecId, BoxDt>` newtype.
///
/// The information may not be of the same type across flows, as flows are
/// different in what they are doing.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(transparent)] // Needed to serialize as a map instead of a list.
pub struct ItemSpecParams(TypeMap<ItemSpecId, BoxDt>);

impl ItemSpecParams {
    /// Returns a new `ItemSpecParams` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `ItemSpecParams` map with the specified capacity.
    ///
    /// The `ItemSpecParams` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity_typed(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<ItemSpecId, BoxDt> {
        self.0
    }
}

impl Deref for ItemSpecParams {
    type Target = TypeMap<ItemSpecId, BoxDt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ItemSpecParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TypeMap<ItemSpecId, BoxDt>> for ItemSpecParams {
    fn from(type_map: TypeMap<ItemSpecId, BoxDt>) -> Self {
        Self(type_map)
    }
}
