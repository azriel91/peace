//! Data types for resource interactions for the Peace framework.

// Re-exports
pub use url::{self, Host, Url};

pub use crate::{
    item_interaction::{
        ItemInteraction, ItemInteractionPull, ItemInteractionPush, ItemInteractionWithin,
    },
    item_interactions_current::ItemInteractionsCurrent,
    item_interactions_current_or_example::ItemInteractionsCurrentOrExample,
    item_interactions_example::ItemInteractionsExample,
    item_location::ItemLocation,
    item_location_ancestors::ItemLocationAncestors,
    item_location_tree::ItemLocationTree,
    item_location_type::ItemLocationType,
};

mod item_interaction;
mod item_interactions_current;
mod item_interactions_current_or_example;
mod item_interactions_example;
mod item_location;
mod item_location_ancestors;
mod item_location_tree;
mod item_location_type;
