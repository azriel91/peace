use dot_ix::model::{
    common::{EdgeId, Edges, NodeHierarchy, NodeId, NodeNames},
    info_graph::{GraphDir, GraphStyle, InfoGraph},
};
use fn_graph::{daggy::Walker, Edge, GraphInfo};
use peace_core::FlowId;

use serde::{Deserialize, Serialize};

use crate::ItemSpecInfo;

/// Serializable representation of how a [`Flow`] is configured.
///
/// [`Flow`]: https://docs.rs/peace_rt_model/latest/peace_rt_model/struct.Flow.html
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FlowSpecInfo {
    /// ID of the flow.
    pub flow_id: FlowId,
    /// Serialized representation of the flow graph.
    pub graph_info: GraphInfo<ItemSpecInfo>,
}

impl FlowSpecInfo {
    /// Returns a new `FlowSpecInfo`.
    pub fn new(flow_id: FlowId, graph_info: GraphInfo<ItemSpecInfo>) -> Self {
        Self {
            flow_id,
            graph_info,
        }
    }

    /// Returns an [`InfoGraph`] that represents the progress of the flow's
    /// execution.
    pub fn to_progress_info_graph(&self) -> InfoGraph {
        let graph_info = &self.graph_info;
        let item_count = graph_info.node_count();

        let hierarchy = graph_info.iter_insertion_with_indices().fold(
            NodeHierarchy::with_capacity(item_count),
            |mut hierarchy, (_node_index, item_spec_info)| {
                let node_id = item_spec_info_to_node_id(item_spec_info);
                // Progress nodes have no nested nodes.
                hierarchy.insert(node_id, NodeHierarchy::new());
                hierarchy
            },
        );

        let edges = progress_node_edges(graph_info);
        let node_names = node_names(graph_info);

        InfoGraph::default()
            .with_graph_style(GraphStyle::Circle)
            .with_direction(GraphDir::Vertical)
            .with_hierarchy(hierarchy)
            .with_edges(edges)
            .with_node_names(node_names)
    }
}

/// Returns the list of edges between items in the graph for progress.
///
/// For progress graphs, an edge is rendered between pairs of predecessor and
/// successor items, regardless of whether their dependency is `Edge::Logic`
/// (adjacent) or `Edge::Contains` (nested).
fn progress_node_edges(graph_info: &GraphInfo<ItemSpecInfo>) -> Edges {
    graph_info.iter_insertion_with_indices().fold(
        Edges::with_capacity(graph_info.node_count()),
        |mut edges, (node_index, item_spec_info)| {
            //
            let children = graph_info.children(node_index);
            children
                .iter(graph_info)
                .filter_map(|(edge_index, child_node_index)| {
                    // For progress graphs, child nodes that:
                    //
                    // * are contained by parents nodes
                    // * reference data from parent nodes
                    //
                    // are both represented by forward edges, since this is their sequential
                    // ordering.
                    if matches!(
                        graph_info.edge_weight(edge_index).copied(),
                        Some(Edge::Logic | Edge::Contains)
                    ) {
                        Some(child_node_index)
                    } else {
                        None
                    }
                })
                .for_each(|child_node_index| {
                    let item_id = item_spec_info_to_node_id(item_spec_info);
                    let child_item_id = item_spec_info_to_node_id(&graph_info[child_node_index]);
                    edges.insert(
                        EdgeId::try_from(format!("{item_id}__{child_item_id}")).expect(
                            "Expected `peace` `ItemId`s concatenated \
                            to be valid `dot_ix` `EdgeId`s.",
                        ),
                        [item_id, child_item_id],
                    );
                });

            edges
        },
    )
}

/// Returns the list of edges between items in the graph.
fn node_names(graph_info: &GraphInfo<ItemSpecInfo>) -> NodeNames {
    graph_info.iter_insertion_with_indices().fold(
        NodeNames::with_capacity(graph_info.node_count()),
        |mut node_names, (_node_index, item_spec_info)| {
            let item_id = item_spec_info_to_node_id(item_spec_info);

            // Note: This does not have to be the ID, it can be a human readable name.
            let node_name = item_id.to_string();

            node_names.insert(item_id, node_name);

            node_names
        },
    )
}

fn item_spec_info_to_node_id(item_spec_info: &ItemSpecInfo) -> NodeId {
    NodeId::try_from(item_spec_info.item_id.to_string())
        .expect("Expected `peace` `ItemId`s to be valid `dot_ix` `NodeId`s.`")
}
