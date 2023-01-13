use crate::outcomes::{ItemEnsure, ItemEnsureRt};

/// A boxed `ItemEnsure`.
#[derive(Clone, serde::Serialize)]
pub struct ItemEnsureBoxed(pub(crate) Box<dyn ItemEnsureRt>);

impl<State, StateDiff> From<ItemEnsure<State, StateDiff>> for ItemEnsureBoxed
where
    ItemEnsure<State, StateDiff>: ItemEnsureRt,
{
    /// Returns an `ItemEnsureBoxed` which erases an `ItemEnsure`'s type
    /// parameters.
    fn from(item_ensure: ItemEnsure<State, StateDiff>) -> Self {
        Self(Box::new(item_ensure))
    }
}

crate::outcomes::box_data_type_newtype!(ItemEnsureBoxed, ItemEnsureRt);
