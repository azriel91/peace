use crate::outcomes::{ItemEnsurePartial, ItemEnsurePartialRt};

/// A boxed `ItemEnsurePartial`.
#[derive(Clone, serde::Serialize)]
pub struct ItemEnsurePartialBoxed(pub(crate) Box<dyn ItemEnsurePartialRt>);

impl<StateLogical, StatePhysical, StateDiff>
    From<ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>> for ItemEnsurePartialBoxed
where
    ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>: ItemEnsurePartialRt,
{
    /// Returns an `ItemEnsurePartialBoxed` which erases an
    /// `ItemEnsurePartial`'s type parameters.
    fn from(item_ensure: ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>) -> Self {
        Self(Box::new(item_ensure))
    }
}

crate::outcomes::box_data_type_newtype!(ItemEnsurePartialBoxed, ItemEnsurePartialRt);
