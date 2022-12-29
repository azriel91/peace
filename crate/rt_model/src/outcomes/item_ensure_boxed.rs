use peace_resources::type_reg::untagged::DataType;

use crate::outcomes::ItemEnsure;

/// A boxed `ItemEnsure`.
#[derive(Clone, serde::Serialize)]
pub struct ItemEnsureBoxed(pub(crate) Box<dyn DataType>);

impl<StateLogical, StatePhysical, StateDiff>
    From<ItemEnsure<StateLogical, StatePhysical, StateDiff>> for ItemEnsureBoxed
where
    ItemEnsure<StateLogical, StatePhysical, StateDiff>: DataType,
{
    /// Returns an `ItemEnsureBoxed` which erases an `ItemEnsure`'s type
    /// parameters.
    fn from(item_ensure: ItemEnsure<StateLogical, StatePhysical, StateDiff>) -> Self {
        Self(Box::new(item_ensure))
    }
}

crate::outcomes::box_data_type_newtype!(ItemEnsureBoxed);
