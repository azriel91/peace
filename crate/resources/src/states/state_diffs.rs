use std::ops::Deref;

use peace_core::ItemId;
use peace_fmt::{Presentable, Presenter};
use serde::Serialize;
use type_reg::untagged::{BoxDtDisplay, TypeMap};

use crate::internal::StateDiffsMut;

/// Diffs of `State`s for each `Item`s. `TypeMap<ItemIdT, BoxDtDisplay>`
/// newtype.
///
/// [`External`] fields are not necessarily used in `StateDiff` computations.
///
/// # Implementors
///
/// [`StateDiffs`] is a read-only resource, stored in [`Resources`] after
/// `DiffCmd` has been executed.
///
/// [`External`]: peace_cfg::state::External
/// [`Resources`]: crate::Resources
#[derive(Debug, Serialize)]
pub struct StateDiffs<ItemIdT>(TypeMap<ItemIdT, BoxDtDisplay>)
where
    ItemIdT: ItemId;

impl<ItemIdT> Default for StateDiffs<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn default() -> Self {
        Self(TypeMap::<ItemIdT, BoxDtDisplay>::default())
    }
}

impl<ItemIdT> StateDiffs<ItemIdT>
where
    ItemIdT: ItemId,
{
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
    pub fn into_inner(self) -> TypeMap<ItemIdT, BoxDtDisplay> {
        self.0
    }
}

impl<ItemIdT> Deref for StateDiffs<ItemIdT>
where
    ItemIdT: ItemId,
{
    type Target = TypeMap<ItemIdT, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<ItemIdT> From<TypeMap<ItemIdT, BoxDtDisplay>> for StateDiffs<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn from(type_map: TypeMap<ItemIdT, BoxDtDisplay>) -> Self {
        Self(type_map)
    }
}

impl<ItemIdT> From<StateDiffsMut<ItemIdT>> for StateDiffs<ItemIdT>
where
    ItemIdT: ItemId,
{
    fn from(state_diffs_mut: StateDiffsMut<ItemIdT>) -> Self {
        Self(state_diffs_mut.into_inner())
    }
}

#[peace_fmt::async_trait(?Send)]
impl<ItemIdT> Presentable for StateDiffs<ItemIdT>
where
    ItemIdT: ItemId + Presentable,
{
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
