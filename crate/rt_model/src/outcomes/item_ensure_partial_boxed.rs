use crate::outcomes::{ItemEnsurePartial, ItemEnsurePartialRt};

/// A boxed `ItemEnsurePartial`.
#[derive(Clone, serde::Serialize)]
pub struct ItemEnsurePartialBoxed(pub(crate) Box<dyn ItemEnsurePartialRt>);

impl<State, StateDiff> From<ItemEnsurePartial<State, StateDiff>> for ItemEnsurePartialBoxed
where
    ItemEnsurePartial<State, StateDiff>: ItemEnsurePartialRt,
{
    /// Returns an `ItemEnsurePartialBoxed` which erases an
    /// `ItemEnsurePartial`'s type parameters.
    fn from(item_ensure: ItemEnsurePartial<State, StateDiff>) -> Self {
        Self(Box::new(item_ensure))
    }
}

crate::outcomes::box_data_type_newtype!(ItemEnsurePartialBoxed, ItemEnsurePartialRt);
