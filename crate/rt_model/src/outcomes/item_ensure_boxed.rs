use crate::outcomes::{ItemEnsure, ItemEnsureRt};

/// A boxed `ItemEnsure`.
#[derive(Clone, serde::Serialize)]
pub struct ItemEnsureBoxed(pub(crate) Box<dyn ItemEnsureRt>);

impl<StateLogical, StatePhysical, StateDiff>
    From<ItemEnsure<StateLogical, StatePhysical, StateDiff>> for ItemEnsureBoxed
where
    ItemEnsure<StateLogical, StatePhysical, StateDiff>: ItemEnsureRt,
{
    /// Returns an `ItemEnsureBoxed` which erases an `ItemEnsure`'s type
    /// parameters.
    fn from(item_ensure: ItemEnsure<StateLogical, StatePhysical, StateDiff>) -> Self {
        Self(Box::new(item_ensure))
    }
}

crate::outcomes::box_data_type_newtype!(ItemEnsureBoxed, ItemEnsureRt);
