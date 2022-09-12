use std::ops::{Deref, DerefMut};

use fn_graph::FnGraphBuilder;

use crate::{ItemSpecBoxed, ItemSpecGraph};

/// Builder for an [`ItemSpecGraph`], `FnGraphBuilder<ItemSpecBoxed<E>>`
/// newtype.
#[derive(Debug)]
pub struct ItemSpecGraphBuilder<E>(FnGraphBuilder<ItemSpecBoxed<E>>);

impl<E> ItemSpecGraphBuilder<E> {
    /// Returns a new `ItemSpecGraphBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner [`FnGraphBuilder`].
    pub fn into_inner(self) -> FnGraphBuilder<ItemSpecBoxed<E>> {
        self.0
    }

    /// Builds and returns the [`ItemSpecGraph`].
    pub fn build(self) -> ItemSpecGraph<E> {
        ItemSpecGraph::from(self.0.build())
    }
}

impl<E> Default for ItemSpecGraphBuilder<E> {
    fn default() -> Self {
        Self(FnGraphBuilder::default())
    }
}

impl<E> Deref for ItemSpecGraphBuilder<E> {
    type Target = FnGraphBuilder<ItemSpecBoxed<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for ItemSpecGraphBuilder<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> From<FnGraphBuilder<ItemSpecBoxed<E>>> for ItemSpecGraphBuilder<E> {
    fn from(graph: FnGraphBuilder<ItemSpecBoxed<E>>) -> Self {
        Self(graph)
    }
}
