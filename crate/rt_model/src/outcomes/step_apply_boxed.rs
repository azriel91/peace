use crate::outcomes::{ItemApply, ItemApplyRt};

/// A boxed `ItemApply`.
#[derive(Clone, serde::Serialize)]
pub struct ItemApplyBoxed(pub(crate) Box<dyn ItemApplyRt>);

impl<State, StateDiff> From<ItemApply<State, StateDiff>> for ItemApplyBoxed
where
    ItemApply<State, StateDiff>: ItemApplyRt,
{
    /// Returns a `StepApplyBoxed` which erases a `StepApply`'s type
    /// parameters.
    fn from(item_apply: ItemApply<State, StateDiff>) -> Self {
        Self(Box::new(item_apply))
    }
}

crate::outcomes::box_data_type_newtype!(ItemApplyBoxed, ItemApplyRt);
