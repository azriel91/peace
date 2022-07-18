use std::ops::{Deref, DerefMut};

use fn_graph::FnGraphBuilder;

use crate::{FullSpecBoxed, FullSpecGraph};

/// Builder for a [`FullSpecGraph`], `FnGraphBuilder<FullSpecBoxed<E>>` newtype.
#[derive(Debug)]
pub struct FullSpecGraphBuilder<E>(FnGraphBuilder<FullSpecBoxed<E>>)
where
    E: std::error::Error;

impl<E> FullSpecGraphBuilder<E>
where
    E: std::error::Error,
{
    /// Returns a new `FullSpecGraphBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner [`FnGraphBuilder`].
    pub fn into_inner(self) -> FnGraphBuilder<FullSpecBoxed<E>> {
        self.0
    }

    /// Builds and returns the [`FullSpecGraph`].
    pub fn build(self) -> FullSpecGraph<E> {
        FullSpecGraph::from(self.0.build())
    }
}

impl<E> Default for FullSpecGraphBuilder<E>
where
    E: std::error::Error,
{
    fn default() -> Self {
        Self(FnGraphBuilder::default())
    }
}

impl<E> Deref for FullSpecGraphBuilder<E>
where
    E: std::error::Error,
{
    type Target = FnGraphBuilder<FullSpecBoxed<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for FullSpecGraphBuilder<E>
where
    E: std::error::Error,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> From<FnGraphBuilder<FullSpecBoxed<E>>> for FullSpecGraphBuilder<E>
where
    E: std::error::Error,
{
    fn from(graph: FnGraphBuilder<FullSpecBoxed<E>>) -> Self {
        Self(graph)
    }
}
