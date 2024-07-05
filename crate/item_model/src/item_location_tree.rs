use serde::{Deserialize, Serialize};

use crate::ItemLocation;

/// An [`ItemLocation`] and its children.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ItemLocationTree {
    /// This [`ItemLocation`].
    pub item_location: ItemLocation,
    /// The children of this [`ItemLocation`].
    pub children: Vec<ItemLocationTree>,
}

impl ItemLocationTree {
    /// Returns a new [`ItemLocationTree`].
    pub fn new(item_location: ItemLocation, children: Vec<ItemLocationTree>) -> Self {
        Self {
            item_location,
            children,
        }
    }

    /// Returns this [`ItemLocation`].
    pub fn item_location(&self) -> &ItemLocation {
        &self.item_location
    }

    /// Returns the children of this [`ItemLocation`].
    pub fn children(&self) -> &[ItemLocationTree] {
        &self.children
    }
}
