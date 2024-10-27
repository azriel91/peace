use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use dot_ix_model::info_graph::InfoGraph;

/// Shared memory for `Map<CmdExecId, InfoGraph>`.
///
/// This may be used for example/actual outcome state.
#[derive(Clone, Debug)]
pub struct FlowInfoGraphs<K>(Arc<Mutex<HashMap<K, InfoGraph>>>);

impl<K> FlowInfoGraphs<K> {
    /// Returns a new `FlowInfoGraphs` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the underlying `Arc<Mutex<HashMap<K, InfoGraph>>>`.
    pub fn into_inner(self) -> Arc<Mutex<HashMap<K, InfoGraph>>> {
        self.0
    }
}

impl<K> Deref for FlowInfoGraphs<K> {
    type Target = Arc<Mutex<HashMap<K, InfoGraph>>>;

    fn deref(&self) -> &Arc<Mutex<HashMap<K, InfoGraph>>> {
        &self.0
    }
}

impl<K> DerefMut for FlowInfoGraphs<K> {
    fn deref_mut(&mut self) -> &mut Arc<Mutex<HashMap<K, InfoGraph>>> {
        &mut self.0
    }
}

impl<K> Default for FlowInfoGraphs<K> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }
}
