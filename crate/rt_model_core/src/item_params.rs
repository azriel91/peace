use std::ops::{Deref, DerefMut};

use peace_core::ItemId;
use serde::Serialize;
use type_reg::untagged::{BoxDt, TypeMap};

/// Map of item ID to its parameters. `TypeMap<ItemId, BoxDt>` newtype.
///
/// The information may not be of the same type across flows, as flows are
/// different in what they are doing.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(transparent)] // Needed to serialize as a map instead of a list.
pub struct ItemParams(TypeMap<ItemId, BoxDt>);

impl ItemParams {
    /// Returns a new `ItemParams` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `ItemParams` map with the specified capacity.
    ///
    /// The `ItemParams` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity_typed(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<ItemId, BoxDt> {
        self.0
    }
}

impl Deref for ItemParams {
    type Target = TypeMap<ItemId, BoxDt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ItemParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TypeMap<ItemId, BoxDt>> for ItemParams {
    fn from(type_map: TypeMap<ItemId, BoxDt>) -> Self {
        Self(type_map)
    }
}
