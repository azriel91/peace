use std::ops::{Deref, DerefMut};

use peace_core::ItemSpecId;
use serde::Serialize;
use type_reg::untagged::{DataType, TypeMap};

/// Diffs of `StateLogical`s for each `ItemSpec`s. `TypeMap<ItemSpecId>`
/// newtype.
///
/// # Implementors
///
/// [`StateDiffsMut`] is a framework-only type and is never inserted into
/// [`Resources`]. If you need to inspect diffs, you may borrow [`StateDiffs`].
///
/// [`StateDiffs`]: crate::StateDiffs
/// [`Resources`]: crate::Resources
#[derive(Debug, Default, Serialize)]
pub struct StateDiffsMut(TypeMap<ItemSpecId>);

impl StateDiffsMut {
    /// Returns a new `StateDiffsMut` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `StateDiffsMut` map with the specified capacity.
    ///
    /// The `StateDiffsMut` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<ItemSpecId> {
        self.0
    }
}

impl Deref for StateDiffsMut {
    type Target = TypeMap<ItemSpecId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StateDiffsMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TypeMap<ItemSpecId>> for StateDiffsMut {
    fn from(type_map: TypeMap<ItemSpecId>) -> Self {
        Self(type_map)
    }
}

impl Extend<(ItemSpecId, Box<dyn DataType>)> for StateDiffsMut {
    fn extend<T: IntoIterator<Item = (ItemSpecId, Box<dyn DataType>)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(item_spec_id, state_diff)| {
            self.insert_raw(item_spec_id, state_diff);
        });
    }
}
