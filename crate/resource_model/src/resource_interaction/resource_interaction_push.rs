use serde::{Deserialize, Serialize};

use crate::ResourceLocation;

/// Represents a location-to-location push interaction.
///
/// This can represent a file transfer from one host to another.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResourceInteractionPush {
    /// Where the interaction begins from.
    ///
    /// e.g.
    ///
    /// 1. `ResourceLocation::localhost()`
    /// 2. `ResourceLocation::new("/path/to/file", ResourceLocationType::Path)`
    pub location_from: Vec<ResourceLocation>,
    /// Where the interaction goes to.
    ///
    /// e.g.
    ///
    /// 1. `ResourceLocation::new("app.domain.com", ResourceLocationType::Host)`
    /// 2. `ResourceLocation::new("http://app.domain.com/resource",
    ///    ResourceLocationType::Path)`
    pub location_to: Vec<ResourceLocation>,
}

impl ResourceInteractionPush {
    /// Returns a new `ResourceInteractionPush`.
    pub fn new(location_from: Vec<ResourceLocation>, location_to: Vec<ResourceLocation>) -> Self {
        Self {
            location_from,
            location_to,
        }
    }

    /// Returns where the interaction begins from.
    ///
    /// e.g.
    ///
    /// 1. `ResourceLocation::localhost()`
    /// 2. `ResourceLocation::new("/path/to/file", ResourceLocationType::Path)`
    pub fn location_from(&self) -> &[ResourceLocation] {
        &self.location_from
    }

    /// Returns where the interaction goes to.
    ///
    /// e.g.
    ///
    /// 1. `ResourceLocation::new("app.domain.com", ResourceLocationType::Host)`
    /// 2. `ResourceLocation::new("http://app.domain.com/resource",
    ///    ResourceLocationType::Path)`
    pub fn location_to(&self) -> &[ResourceLocation] {
        &self.location_to
    }
}
