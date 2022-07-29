use std::ops::{Deref, DerefMut};

use peace_core::FullSpecId;
use serde::Serialize;
use type_reg::untagged::{DataType, TypeMap};

/// Desired `State`s for all `FullSpec`s. `TypeMap<FullSpecId>` newtype.
///
/// # Implementors
///
/// * In `StateNowFnSpec`, you should reference [`StatesDesiredRw`], which
///   allows mutable access to the underlying desired states.
/// * In `EnsureOpSpec`, you should reference [`StatesDesired`].
/// * [`StatesDesiredMut`] is not intended to be referenced in [`Data`]
///   directly.
///
/// You may reference [`StatesDesired`] in `EnsureOpSpec::Data` for reading. It
/// is not mutable as `State` must remain unchanged so that all `FullSpec`s
/// operate over consistent data.
///
/// [`Data`]: peace_data::Data
/// [`StatesDesired`]: crate::StatesDesired
/// [`StatesDesiredRw`]: crate::StatesDesiredRw
/// [`Resources`]: crate::Resources
#[derive(Debug, Default, Serialize)]
pub struct StatesDesiredMut(TypeMap<FullSpecId>);

impl StatesDesiredMut {
    /// Returns a new `StatesDesiredMut` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `StatesDesiredMut` map with the specified capacity.
    ///
    /// The `StatesDesiredMut` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<FullSpecId> {
        self.0
    }
}

impl Deref for StatesDesiredMut {
    type Target = TypeMap<FullSpecId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatesDesiredMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TypeMap<FullSpecId>> for StatesDesiredMut {
    fn from(type_map: TypeMap<FullSpecId>) -> Self {
        Self(type_map)
    }
}

impl Extend<(FullSpecId, Box<dyn DataType>)> for StatesDesiredMut {
    fn extend<T: IntoIterator<Item = (FullSpecId, Box<dyn DataType>)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(full_spec_id, state_desired)| {
            self.insert_raw(full_spec_id, state_desired);
        });
    }
}
