use serde::{Deserialize, Serialize};

use crate::ItemLocation;

/// Represents a location-to-location push interaction.
///
/// This can represent a file transfer from one host to another.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResourceInteractionPush {
    /// Where the interaction begins from.
    ///
    /// e.g.
    ///
    /// 1. `ItemLocation::localhost()`
    /// 2. `ItemLocation::new("/path/to/file", ItemLocationType::Path)`
    pub location_from: Vec<ItemLocation>,
    /// Where the interaction goes to.
    ///
    /// e.g.
    ///
    /// 1. `ItemLocation::new("app.domain.com", ItemLocationType::Host)`
    /// 2. `ItemLocation::new("http://app.domain.com/resource",
    ///    ItemLocationType::Path)`
    pub location_to: Vec<ItemLocation>,
}

impl ResourceInteractionPush {
    /// Returns a new `ResourceInteractionPush`.
    pub fn new(location_from: Vec<ItemLocation>, location_to: Vec<ItemLocation>) -> Self {
        Self {
            location_from,
            location_to,
        }
    }

    /// Returns where the interaction begins from.
    ///
    /// e.g.
    ///
    /// 1. `ItemLocation::localhost()`
    /// 2. `ItemLocation::new("/path/to/file", ItemLocationType::Path)`
    pub fn location_from(&self) -> &[ItemLocation] {
        &self.location_from
    }

    /// Returns where the interaction goes to.
    ///
    /// e.g.
    ///
    /// 1. `ItemLocation::new("app.domain.com", ItemLocationType::Host)`
    /// 2. `ItemLocation::new("http://app.domain.com/resource",
    ///    ItemLocationType::Path)`
    pub fn location_to(&self) -> &[ItemLocation] {
        &self.location_to
    }
}
