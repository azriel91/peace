use std::ops::{Deref, DerefMut};

use crate::FlowInfoGraphs;

/// Shared memory for `Map<CmdExecId, InfoGraph>`.
///
/// This is intended to be used for progress diagrams.
#[derive(Clone, Debug)]
pub struct FlowProgressInfoGraphs<K>(FlowInfoGraphs<K>);

impl<K> FlowProgressInfoGraphs<K> {
    /// Returns a new `FlowProgressInfoGraphs` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the underlying `FlowInfoGraphs<K>`.
    pub fn into_inner(self) -> FlowInfoGraphs<K> {
        self.0
    }
}

impl<K> Deref for FlowProgressInfoGraphs<K> {
    type Target = FlowInfoGraphs<K>;

    fn deref(&self) -> &FlowInfoGraphs<K> {
        &self.0
    }
}

impl<K> DerefMut for FlowProgressInfoGraphs<K> {
    fn deref_mut(&mut self) -> &mut FlowInfoGraphs<K> {
        &mut self.0
    }
}

impl<K> Default for FlowProgressInfoGraphs<K> {
    fn default() -> Self {
        Self(Default::default())
    }
}
