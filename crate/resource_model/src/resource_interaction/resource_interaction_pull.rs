use serde::{Deserialize, Serialize};

use crate::ResourceLocation;

/// Represents a location-to-location pull interaction.
///
/// This can represent a file download from a server.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResourceInteractionPull {
    /// Where the interaction begins from.
    ///
    /// e.g.
    ///
    /// 1. `ResourceLocation::localhost()`
    /// 2. `ResourceLocation::new("/path/to/file", ResourceLocationType::Path)`
    pub location_client: Vec<ResourceLocation>,
    /// Where the interaction goes to.
    ///
    /// e.g.
    ///
    /// 1. `ResourceLocation::new("app.domain.com", ResourceLocationType::Host)`
    /// 2. `ResourceLocation::new("http://app.domain.com/resource",
    ///    ResourceLocationType::Path)`
    pub location_server: Vec<ResourceLocation>,
}

impl ResourceInteractionPull {
    /// Returns a new `ResourceInteractionPull`.
    pub fn new(
        location_client: Vec<ResourceLocation>,
        location_server: Vec<ResourceLocation>,
    ) -> Self {
        Self {
            location_client,
            location_server,
        }
    }

    /// Returns where the interaction begins from.
    ///
    /// e.g.
    ///
    /// 1. `ResourceLocation::localhost()`
    /// 2. `ResourceLocation::new("/path/to/file", ResourceLocationType::Path)`
    pub fn location_client(&self) -> &[ResourceLocation] {
        &self.location_client
    }

    /// Returns where the interaction goes to.
    ///
    /// e.g.
    ///
    /// 1. `ResourceLocation::new("app.domain.com", ResourceLocationType::Host)`
    /// 2. `ResourceLocation::new("http://app.domain.com/resource",
    ///    ResourceLocationType::Path)`
    pub fn location_server(&self) -> &[ResourceLocation] {
        &self.location_server
    }
}
