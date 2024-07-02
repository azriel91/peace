//! Data types for resource interactions for the Peace framework.

// Re-exports
pub use url::{self, Host, Url};

pub use crate::{
    resource_interaction::{
        ItemInteraction, ItemInteractionPull, ItemInteractionPush,
        ItemInteractionWithin,
    },
    item_location::ItemLocation,
    item_location_type::ItemLocationType,
};

mod resource_interaction;
mod item_location;
mod item_location_type;
