use serde::{Deserialize, Serialize};

use crate::ItemLocation;

/// Represents a resource interaction that happens within a location.
///
/// This can represent application installation / startup happening on a
/// server.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResourceInteractionWithin {
    /// Where the interaction is happening.
    ///
    /// e.g.
    ///
    /// 1. `ItemLocation::Server { address, port: None }`
    pub location: Vec<ItemLocation>,
}

impl ResourceInteractionWithin {
    /// Returns a new `ResourceInteractionWithin`.
    pub fn new(location: Vec<ItemLocation>) -> Self {
        Self { location }
    }

    /// Returns where the interaction is happening.
    pub fn location(&self) -> &[ItemLocation] {
        &self.location
    }
}
