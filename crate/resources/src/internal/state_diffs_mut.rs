use std::ops::{Deref, DerefMut};

use peace_core::ItemId;
use serde::Serialize;
use type_reg::untagged::{BoxDtDisplay, TypeMap};

/// Diffs of `State`s for each `Item`s. `TypeMap<ItemIdT, BoxDtDisplay>`
/// newtype.
///
/// # Implementors
///
/// [`StateDiffsMut`] is a framework-only type and is never inserted into
/// [`Resources`]. If you need to inspect diffs, you may borrow [`StateDiffs`].
///
/// [`StateDiffs`]: crate::StateDiffs
/// [`Resources`]: crate::Resources
#[derive(Debug, Serialize)]
pub struct StateDiffsMut<ItemIdT>(TypeMap<ItemIdT, BoxDtDisplay>)
where
    ItemIdT: ItemId;

impl<ItemIdT> StateDiffsMut<ItemIdT>
where
    ItemIdT: ItemId,
{
    /// Returns a new `StateDiffsMut` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `StateDiffsMut` map with the specified capacity.
    ///
    /// The `StateDiffsMut` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity_typed(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<ItemIdT, BoxDtDisplay> {
        self.0
    }
}

impl<ItemIdT> Default for StateDiffsMut<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn default() -> Self {
        Self(TypeMap::default())
    }
}

impl<ItemIdT> Deref for StateDiffsMut<ItemIdT>
where
    ItemIdT: ItemId,
{
    type Target = TypeMap<ItemIdT, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<ItemIdT> DerefMut for StateDiffsMut<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<ItemIdT> From<TypeMap<ItemIdT, BoxDtDisplay>> for StateDiffsMut<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn from(type_map: TypeMap<ItemIdT, BoxDtDisplay>) -> Self {
        Self(type_map)
    }
}

impl<ItemIdT> Extend<(ItemIdT, BoxDtDisplay)> for StateDiffsMut<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn extend<T: IntoIterator<Item = (ItemIdT, BoxDtDisplay)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(item_id, state_diff)| {
            self.insert_raw(item_id, state_diff);
        });
    }
}
