use serde::{Deserialize, Serialize};

/// Marker for example state.
///
/// This is used for referential param values, where an item param value is
/// dependent on the state of a predecessor's state.
///
/// An `Example<Item::State>` is set to `Some` whenever an item's example state
/// is discovered. This is used for rendering outcome diagrams in the following
/// cases:
///
/// 1. Rendering an example fully deployed state.
/// 2. Rendering invisible placeholder nodes and edges, so that the layout of a
///    diagram is consistent as more items are applied.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Example<T>(pub Option<T>);

impl<T> std::ops::Deref for Example<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Example<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
