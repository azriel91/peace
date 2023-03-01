use std::ops::{Deref, DerefMut};

use fn_graph::FnGraph;

use crate::ItemSpecBoxed;

/// Graph of all [`ItemSpec`]s, `FnGraph<ItemSpecBoxed<E>>` newtype.
///
/// [`ItemSpec`]: peace_cfg::ItemSpec
#[derive(Debug)]
pub struct ItemSpecGraph<E>(FnGraph<ItemSpecBoxed<E>>);

// Manual implementation because derive requires `E` to be `Clone`,
// which causes `graph.clone()` to call `FnGraph::clone`.
impl<E> Clone for ItemSpecGraph<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<E> ItemSpecGraph<E> {
    /// Returns the inner [`FnGraph`].
    pub fn into_inner(self) -> FnGraph<ItemSpecBoxed<E>> {
        self.0
    }
}

impl<E> Deref for ItemSpecGraph<E> {
    type Target = FnGraph<ItemSpecBoxed<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for ItemSpecGraph<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> From<FnGraph<ItemSpecBoxed<E>>> for ItemSpecGraph<E> {
    fn from(graph: FnGraph<ItemSpecBoxed<E>>) -> Self {
        Self(graph)
    }
}
