//! Data types for resource interactions for the Peace framework.

// Re-exports
pub use url::Host;

pub use crate::{
    resource_interaction::{
        ResourceInteraction, ResourceInteractionPull, ResourceInteractionPush,
        ResourceInteractionWithin,
    },
    resource_location::ResourceLocation,
    resource_location_type::ResourceLocationType,
};

mod resource_interaction;
mod resource_location;
mod resource_location_type;
