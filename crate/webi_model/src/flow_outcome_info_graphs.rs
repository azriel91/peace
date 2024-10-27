use std::ops::{Deref, DerefMut};

use crate::FlowInfoGraphs;

/// Shared memory for `Map<CmdExecId, InfoGraph>`.
///
/// This is intended to be used for example / actual outcome diagrams.
#[derive(Clone, Debug)]
pub struct FlowOutcomeInfoGraphs<K>(FlowInfoGraphs<K>);

impl<K> FlowOutcomeInfoGraphs<K> {
    /// Returns a new `FlowOutcomeInfoGraphs` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the underlying `FlowInfoGraphs<K>`.
    pub fn into_inner(self) -> FlowInfoGraphs<K> {
        self.0
    }
}

impl<K> Deref for FlowOutcomeInfoGraphs<K> {
    type Target = FlowInfoGraphs<K>;

    fn deref(&self) -> &FlowInfoGraphs<K> {
        &self.0
    }
}

impl<K> DerefMut for FlowOutcomeInfoGraphs<K> {
    fn deref_mut(&mut self) -> &mut FlowInfoGraphs<K> {
        &mut self.0
    }
}

impl<K> Default for FlowOutcomeInfoGraphs<K> {
    fn default() -> Self {
        Self(Default::default())
    }
}
