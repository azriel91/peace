use std::ops::Deref;

use peace_core::ItemId;
use peace_fmt::{Presentable, Presenter};
use serde::Serialize;
use type_reg::untagged::{BoxDtDisplay, TypeMap};

use crate::internal::StateDiffsMut;

/// Diffs of `State`s for each `Item`s. `TypeMap<ItemId, BoxDtDisplay>`
/// newtype.
///
/// # Implementors
///
/// [`StateDiffs`] is a read-only resource, stored in [`Resources`] after
/// `DiffCmd` has been executed.
///
/// [`Resources`]: crate::Resources
#[derive(Debug, Default, Serialize)]
pub struct StateDiffs(TypeMap<ItemId, BoxDtDisplay>);

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
        Self(TypeMap::with_capacity_typed(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<ItemId, BoxDtDisplay> {
        self.0
    }
}

impl Deref for StateDiffs {
    type Target = TypeMap<ItemId, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TypeMap<ItemId, BoxDtDisplay>> for StateDiffs {
    fn from(type_map: TypeMap<ItemId, BoxDtDisplay>) -> Self {
        Self(type_map)
    }
}

impl From<StateDiffsMut> for StateDiffs {
    fn from(states_goal_mut: StateDiffsMut) -> Self {
        Self(states_goal_mut.into_inner())
    }
}

#[peace_fmt::async_trait(?Send)]
impl Presentable for StateDiffs {
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter
            .list_numbered_with(self.iter(), |(item_id, state_diff)| {
                (item_id, format!(": {state_diff}"))
            })
            .await
    }
}
