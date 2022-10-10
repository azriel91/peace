use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use peace_core::ItemSpecId;
use serde::Serialize;
use type_reg::untagged::{BoxDtDisplay, TypeMap};

/// `State`s for all `ItemSpec`s. `TypeMap<ItemSpecId, BoxDtDisplay>` newtype.
///
/// # Implementors
///
/// * In `StateCurrentFnSpec`, you should reference [`StatesRw`], which allows
///   mutable access to the underlying states.
/// * In `EnsureOpSpec`, you should reference [`StatesCurrent`].
/// * [`StatesMut`] is not intended to be referenced in [`Data`] directly.
///
/// You may reference [`StatesCurrent`] in `EnsureOpSpec::Data` for reading. It
/// is not mutable as `State` must remain unchanged so that all `ItemSpec`s
/// operate over consistent data.
///
/// # Type Parameters
///
/// * `TS`: Type state to distinguish the purpose of the `States` map.
///
/// [`Data`]: peace_data::Data
/// [`StatesCurrent`]: crate::StatesCurrent
/// [`StatesRw`]: crate::StatesRw
/// [`Resources`]: crate::Resources
#[derive(Debug, Serialize)]
pub struct StatesMut<TS>(TypeMap<ItemSpecId, BoxDtDisplay>, PhantomData<TS>);

impl<TS> StatesMut<TS> {
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
    pub fn into_inner(self) -> TypeMap<ItemSpecId, BoxDtDisplay> {
        self.0
    }
}

impl<TS> Default for StatesMut<TS> {
    fn default() -> Self {
        Self(TypeMap::default(), PhantomData)
    }
}

impl<TS> Deref for StatesMut<TS> {
    type Target = TypeMap<ItemSpecId, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<TS> DerefMut for StatesMut<TS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<TS> From<TypeMap<ItemSpecId, BoxDtDisplay>> for StatesMut<TS> {
    fn from(type_map: TypeMap<ItemSpecId, BoxDtDisplay>) -> Self {
        Self(type_map, PhantomData)
    }
}

impl<TS> Extend<(ItemSpecId, BoxDtDisplay)> for StatesMut<TS> {
    fn extend<T: IntoIterator<Item = (ItemSpecId, BoxDtDisplay)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(item_spec_id, state)| {
            self.insert_raw(item_spec_id, state);
        });
    }
}
