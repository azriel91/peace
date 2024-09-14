use indexmap::IndexMap;
use peace_core::ItemId;
use serde::{Deserialize, Serialize};

use crate::{ItemInteraction, ItemLocationTree};

/// Merged [`ItemLocation`]s and [`ItemInteraction`]s from all items.
///
/// [`ItemLocation`]: crate::ItemLocation
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ItemLocationsAndInteractions {
    /// Hierachical storage of [`ItemLocation`]s.
    ///
    /// [`ItemLocation`]: crate::ItemLocation
    pub item_location_trees: Vec<ItemLocationTree>,
    /// The [`ItemInteraction`]s from each item.
    pub item_to_item_interactions: IndexMap<ItemId, Vec<ItemInteraction>>,
}

impl ItemLocationsAndInteractions {
    /// Returns a new `ItemLocationsAndInteractions` container.
    pub fn new(
        item_location_trees: Vec<ItemLocationTree>,
        item_to_item_interactions: IndexMap<ItemId, Vec<ItemInteraction>>,
    ) -> Self {
        Self {
            item_location_trees,
            item_to_item_interactions,
        }
    }

    /// Returns the hierachical storage of [`ItemLocation`]s.
    ///
    /// [`ItemLocation`]: crate::ItemLocation
    pub fn item_location_trees(&self) -> &[ItemLocationTree] {
        &self.item_location_trees
    }

    /// Returns the [`ItemInteraction`]s from each item.
    pub fn item_to_item_interactions(&self) -> &IndexMap<ItemId, Vec<ItemInteraction>> {
        &self.item_to_item_interactions
    }
}
