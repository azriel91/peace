//! Data types for resource interactions for the Peace framework.

// Re-exports
pub use url::{self, Host, Url};

pub use crate::{
    resource_interaction::{
        ResourceInteraction, ResourceInteractionPull, ResourceInteractionPush,
        ResourceInteractionWithin,
    },
    resource_location::ItemLocation,
    resource_location_type::ItemLocationType,
};

mod resource_interaction;
mod resource_location;
mod resource_location_type;
