use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use peace_data::fn_graph::FnGraph;
use peace_resources::states::{States, StatesSerde};

use crate::StepBoxed;

/// Graph of all [`Step`]s, `FnGraph<StepBoxed<E>>` newtype.
///
/// [`Step`]: peace_cfg::Step
#[derive(Debug)]
pub struct StepGraph<E>(FnGraph<StepBoxed<E>>);

// Manual implementation because derive requires `E` to be `Clone`,
// which causes `graph.clone()` to call `FnGraph::clone`.
impl<E> Clone for StepGraph<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<E> PartialEq for StepGraph<E>
where
    E: 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<E> Eq for StepGraph<E> where E: 'static {}

impl<E> StepGraph<E> {
    /// Returns the inner [`FnGraph`].
    pub fn into_inner(self) -> FnGraph<StepBoxed<E>> {
        self.0
    }

    /// Returns a user-friendly serializable states map.
    ///
    /// This will contain an entry for all steps, in order of flow step
    /// insertion, whether or not a state exists in the provided `states` map.
    pub fn states_serde<ValueT, TS>(&self, states: &States<TS>) -> StatesSerde<ValueT>
    where
        ValueT: Clone + Debug + PartialEq + Eq,
        E: 'static,
    {
        StatesSerde::from_iter(self.0.iter_insertion().map(|step| {
            let step_id = step.id();
            (step_id.clone(), states.get_raw(step_id).cloned())
        }))
    }
}

impl<E> Deref for StepGraph<E> {
    type Target = FnGraph<StepBoxed<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for StepGraph<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> From<FnGraph<StepBoxed<E>>> for StepGraph<E> {
    fn from(graph: FnGraph<StepBoxed<E>>) -> Self {
        Self(graph)
    }
}

impl<'graph, ValueT, E> From<&'graph StepGraph<E>> for StatesSerde<ValueT>
where
    ValueT: Clone + Debug + PartialEq + Eq,
    E: 'static,
{
    fn from(graph: &'graph StepGraph<E>) -> Self {
        StatesSerde::from_iter(graph.iter_insertion().map(|step| (step.id().clone(), None)))
    }
}
