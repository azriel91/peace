use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use peace_core::ItemId;
use serde::Serialize;
use type_reg::untagged::{BoxDtDisplay, TypeMap};

/// `State`s for all `Item`s. `TypeMap<ItemIdT, BoxDtDisplay>` newtype.
///
/// # Implementors
///
/// To reference State from another `Item`, in `Item::Data`, you should
/// reference [`Current<T>`] or [`Goal<T>`], where `T` is the predecessor
/// item's state.
///
/// # Type Parameters
///
/// * `TS`: Type state to distinguish the purpose of the `States` map.
///
/// [`Current<T>`]: peace_data::marker::Current
/// [`Data`]: peace_data::Data
/// [`Goal<T>`]: peace_data::marker::Goal
/// [`Resources`]: crate::Resources
/// [`StatesCurrent`]: crate::StatesCurrent
/// [`StatesRw`]: crate::StatesRw
#[derive(Debug, Serialize)]
pub struct StatesMut<ItemIdT, TS>(TypeMap<ItemIdT, BoxDtDisplay>, PhantomData<TS>)
where
    ItemIdT: ItemId;

impl<ItemIdT, TS> StatesMut<ItemIdT, TS>
where
    ItemIdT: ItemId,
{
    /// Returns a new `StatesMut` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `StatesMut` map with the specified capacity.
    ///
    /// The `StatesMut` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity_typed(capacity), PhantomData)
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<ItemIdT, BoxDtDisplay> {
        self.0
    }
}

impl<ItemIdT, TS> Default for StatesMut<ItemIdT, TS>
where
    ItemIdT: ItemId,
{
    fn default() -> Self {
        Self(TypeMap::default(), PhantomData)
    }
}

impl<ItemIdT, TS> Deref for StatesMut<ItemIdT, TS>
where
    ItemIdT: ItemId,
{
    type Target = TypeMap<ItemIdT, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<ItemIdT, TS> DerefMut for StatesMut<ItemIdT, TS>
where
    ItemIdT: ItemId,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<ItemIdT, TS> From<TypeMap<ItemIdT, BoxDtDisplay>> for StatesMut<ItemIdT, TS>
where
    ItemIdT: ItemId,
{
    fn from(type_map: TypeMap<ItemIdT, BoxDtDisplay>) -> Self {
        Self(type_map, PhantomData)
    }
}

impl<ItemIdT, TS> Extend<(ItemIdT, BoxDtDisplay)> for StatesMut<ItemIdT, TS>
where
    ItemIdT: ItemId,
{
    fn extend<T: IntoIterator<Item = (ItemIdT, BoxDtDisplay)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(item_id, state)| {
            self.insert_raw(item_id, state);
        });
    }
}
