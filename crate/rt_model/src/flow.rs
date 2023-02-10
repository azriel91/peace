use peace_cfg::FlowId;

use crate::ItemSpecGraph;

/// A flow to manage items.
///
/// A Flow ID is strictly associated with an [`ItemSpecGraph`], as the graph
/// contains the definitions to read and write the items' [`State`]s.
///
/// [`State`]: peace_cfg::ItemSpec::State
#[derive(Debug)]
pub struct Flow<E> {
    /// ID of this flow.
    flow_id: FlowId,
    /// Graph of [`ItemSpec`]s in this flow.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    graph: ItemSpecGraph<E>,
}

impl<E> Flow<E> {
    /// Returns a new `Flow`.
    pub fn new(flow_id: FlowId, graph: ItemSpecGraph<E>) -> Self {
        Self { flow_id, graph }
    }

    /// Returns the flow ID.
    pub fn flow_id(&self) -> &FlowId {
        &self.flow_id
    }

    /// Returns the flow item spec graph.
    pub fn graph(&self) -> &ItemSpecGraph<E> {
        &self.graph
    }
}
