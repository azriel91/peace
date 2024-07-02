use serde::{Deserialize, Serialize};

mod resource_interaction_pull;
mod resource_interaction_push;
mod resource_interaction_within;

pub use self::{
    resource_interaction_pull::ResourceInteractionPull,
    resource_interaction_push::ResourceInteractionPush,
    resource_interaction_within::ResourceInteractionWithin,
};

/// Represents the resources that are read from / written to.
///
/// This is used on an outcome diagram to highlight the resources that are being
/// accessed. For example, a file is read from the user's computer, and uploaded
/// / written to a file server.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ResourceInteraction {
    /// Represents a location-to-location push interaction.
    ///
    /// This can represent a file transfer from one host to another.
    Push(ResourceInteractionPush),
    /// Represents a location-to-location pull interaction.
    ///
    /// This can represent a file download from a server.
    Pull(ResourceInteractionPull),
    /// Represents a resource interaction that happens within a location.
    ///
    /// This can represent application installation / startup happening on a
    /// server.
    Within(ResourceInteractionWithin),
}

impl From<ResourceInteractionPush> for ResourceInteraction {
    fn from(resource_interaction_push: ResourceInteractionPush) -> Self {
        Self::Push(resource_interaction_push)
    }
}

impl From<ResourceInteractionPull> for ResourceInteraction {
    fn from(resource_interaction_pull: ResourceInteractionPull) -> Self {
        Self::Pull(resource_interaction_pull)
    }
}

impl From<ResourceInteractionWithin> for ResourceInteraction {
    fn from(resource_interaction_within: ResourceInteractionWithin) -> Self {
        Self::Within(resource_interaction_within)
    }
}
