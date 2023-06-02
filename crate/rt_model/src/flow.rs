use peace_cfg::FlowId;

use crate::ItemGraph;

/// A flow to manage items.
///
/// A Flow ID is strictly associated with an [`ItemGraph`], as the graph
/// contains the definitions to read and write the items' [`State`]s.
///
/// [`State`]: peace_cfg::Item::State
#[derive(Debug)]
pub struct Flow<E> {
    /// ID of this flow.
    flow_id: FlowId,
    /// Graph of [`Item`]s in this flow.
    ///
    /// [`Item`]: peace_cfg::Item
    graph: ItemGraph<E>,
}

impl<E> PartialEq for Flow<E>
where
    E: 'static,
{
    fn eq(&self, other: &Flow<E>) -> bool {
        self.flow_id == other.flow_id && self.graph == other.graph
    }
}

impl<E> Clone for Flow<E> {
    fn clone(&self) -> Self {
        Self {
            flow_id: self.flow_id.clone(),
            graph: self.graph.clone(),
        }
    }
}

impl<E> Eq for Flow<E> where E: 'static {}

impl<E> Flow<E> {
    /// Returns a new `Flow`.
    pub fn new(flow_id: FlowId, graph: ItemGraph<E>) -> Self {
        Self { flow_id, graph }
    }

    /// Returns the flow ID.
    pub fn flow_id(&self) -> &FlowId {
        &self.flow_id
    }

    /// Returns the item graph.
    pub fn graph(&self) -> &ItemGraph<E> {
        &self.graph
    }

    /// Returns a mutable reference to the item graph.
    pub fn graph_mut(&self) -> &ItemGraph<E> {
        &self.graph
    }
}
