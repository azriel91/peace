use std::ops::{Deref, DerefMut};

use fn_graph::FnGraphBuilder;

use crate::{ItemSpecBoxed, ItemSpecGraph};

/// Builder for an [`ItemSpecGraph`], `FnGraphBuilder<ItemSpecBoxed<E>>`
/// newtype.
#[derive(Debug)]
pub struct ItemSpecGraphBuilder<E>(FnGraphBuilder<ItemSpecBoxed<E>>)
where
    E: std::error::Error;

impl<E> ItemSpecGraphBuilder<E>
where
    E: std::error::Error,
{
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

impl<E> Default for ItemSpecGraphBuilder<E>
where
    E: std::error::Error,
{
    fn default() -> Self {
        Self(FnGraphBuilder::default())
    }
}

impl<E> Deref for ItemSpecGraphBuilder<E>
where
    E: std::error::Error,
{
    type Target = FnGraphBuilder<ItemSpecBoxed<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for ItemSpecGraphBuilder<E>
where
    E: std::error::Error,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> From<FnGraphBuilder<ItemSpecBoxed<E>>> for ItemSpecGraphBuilder<E>
where
    E: std::error::Error,
{
    fn from(graph: FnGraphBuilder<ItemSpecBoxed<E>>) -> Self {
        Self(graph)
    }
}
