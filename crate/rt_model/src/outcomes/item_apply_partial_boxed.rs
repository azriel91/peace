use crate::outcomes::{ItemApplyPartial, ItemApplyPartialRt};

/// A boxed `ItemApplyPartial`.
#[derive(Clone, serde::Serialize)]
pub struct ItemApplyPartialBoxed(pub(crate) Box<dyn ItemApplyPartialRt>);

impl<State, StateDiff> From<ItemApplyPartial<State, StateDiff>> for ItemApplyPartialBoxed
where
    ItemApplyPartial<State, StateDiff>: ItemApplyPartialRt,
{
    /// Returns an `ItemApplyPartialBoxed` which erases an
    /// `ItemApplyPartial`'s type parameters.
    fn from(item_apply: ItemApplyPartial<State, StateDiff>) -> Self {
        Self(Box::new(item_apply))
    }
}

crate::outcomes::box_data_type_newtype!(ItemApplyPartialBoxed, ItemApplyPartialRt);
