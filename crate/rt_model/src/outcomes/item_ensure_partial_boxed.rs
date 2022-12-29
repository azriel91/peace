use peace_resources::type_reg::untagged::DataType;

use crate::outcomes::ItemEnsurePartial;

/// A boxed `ItemEnsurePartial`.
#[derive(Clone, serde::Serialize)]
pub struct ItemEnsurePartialBoxed(pub(crate) Box<dyn DataType>);

impl<StateLogical, StatePhysical, StateDiff>
    From<ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>> for ItemEnsurePartialBoxed
where
    ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>: DataType,
{
    /// Returns an `ItemEnsurePartialBoxed` which erases an
    /// `ItemEnsurePartial`'s type parameters.
    fn from(item_ensure: ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>) -> Self {
        Self(Box::new(item_ensure))
    }
}

crate::outcomes::box_data_type_newtype!(ItemEnsurePartialBoxed);
