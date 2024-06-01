use std::ops::{Deref, DerefMut};

use peace_core::StepId;
use serde::Serialize;
use type_reg::untagged::{BoxDtDisplay, TypeMap};

/// Diffs of `State`s for each `Step`s. `TypeMap<StepId, BoxDtDisplay>`
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
pub struct StateDiffsMut(TypeMap<StepId, BoxDtDisplay>);

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
        Self(TypeMap::with_capacity_typed(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<StepId, BoxDtDisplay> {
        self.0
    }
}

impl Deref for StateDiffsMut {
    type Target = TypeMap<StepId, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StateDiffsMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TypeMap<StepId, BoxDtDisplay>> for StateDiffsMut {
    fn from(type_map: TypeMap<StepId, BoxDtDisplay>) -> Self {
        Self(type_map)
    }
}

impl Extend<(StepId, BoxDtDisplay)> for StateDiffsMut {
    fn extend<T: IntoIterator<Item = (StepId, BoxDtDisplay)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(step_id, state_diff)| {
            self.insert_raw(step_id, state_diff);
        });
    }
}
