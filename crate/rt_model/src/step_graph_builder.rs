use std::ops::{Deref, DerefMut};

use peace_data::fn_graph::FnGraphBuilder;

use crate::{StepBoxed, StepGraph};

/// Builder for a [`StepGraph`], `FnGraphBuilder<StepBoxed<E>>`
/// newtype.
#[derive(Debug)]
pub struct StepGraphBuilder<E>(FnGraphBuilder<StepBoxed<E>>);

impl<E> StepGraphBuilder<E> {
    /// Returns a new `StepGraphBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner [`FnGraphBuilder`].
    pub fn into_inner(self) -> FnGraphBuilder<StepBoxed<E>> {
        self.0
    }

    /// Builds and returns the [`StepGraph`].
    pub fn build(self) -> StepGraph<E> {
        StepGraph::from(self.0.build())
    }
}

impl<E> Default for StepGraphBuilder<E> {
    fn default() -> Self {
        Self(FnGraphBuilder::default())
    }
}

impl<E> Deref for StepGraphBuilder<E> {
    type Target = FnGraphBuilder<StepBoxed<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for StepGraphBuilder<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> From<FnGraphBuilder<StepBoxed<E>>> for StepGraphBuilder<E> {
    fn from(graph: FnGraphBuilder<StepBoxed<E>>) -> Self {
        Self(graph)
    }
}
