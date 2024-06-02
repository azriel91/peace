use peace_core::ItemId;
use serde::{Deserialize, Serialize};

/// Serializable representation of how a [`Step`] is configured.
///
/// [`Item`]: https://docs.rs/peace_cfg/latest/peace_cfg/trait.Item.html
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ItemSpecInfo {
    /// ID of the `Item`.
    pub item_id: ItemId,
}

impl ItemSpecInfo {
    /// Returns a new `ItemSpecInfo`.
    pub fn new(item_id: ItemId) -> Self {
        Self { item_id }
    }
}
