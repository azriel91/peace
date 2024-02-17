use std::collections::HashSet;

use dot_ix::model::{
    common::{EdgeId, NodeHierarchy, NodeId},
    info_graph::{GraphDir, IndexMap, InfoGraph, NodeInfo},
};
use fn_graph::{daggy::Walker, Edge, FnId, GraphInfo};
use peace_core::FlowId;

use serde::{Deserialize, Serialize};

use crate::ItemSpecInfo;

/// Serializable representation of how a [`Flow`] is configured.
///
/// [`Flow`]: https://docs.rs/peace_rt_model/latest/peace_rt_model/struct.Flow.html
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FlowSpecInfo {
    /// ID of the flow.
    pub flow_id: FlowId,
    /// Serialized representation of the flow graph.
    pub graph_info: GraphInfo<ItemSpecInfo>,
}

impl FlowSpecInfo {
    /// Returns an [`InfoGraph`] that represents the progress of the flow's
    /// execution.
    pub fn into_progress_info_graph(&self) -> InfoGraph {
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
        let node_infos = node_infos(graph_info);

        InfoGraph::builder()
            .with_direction(GraphDir::Vertical)
            .with_hierarchy(hierarchy)
            .with_edges(edges)
            .with_node_infos(node_infos)
            .build()
    }

    /// Returns an [`InfoGraph`] that represents the outcome of the flow's
    /// execution.
    pub fn into_outcome_info_graph(&self) -> InfoGraph {
        let graph_info = &self.graph_info;
        let item_count = graph_info.node_count();

        let mut visited = HashSet::with_capacity(item_count);
        let visited = &mut visited;
        let hierarchy = graph_info
            .iter_insertion_with_indices()
            .filter_map(|(node_index, item_spec_info)| {
                let node_hierarchy = outcome_node_hierarchy(graph_info, visited, node_index);
                let node_id = item_spec_info_to_node_id(item_spec_info);
                node_hierarchy.map(|node_hierarchy| (node_id, node_hierarchy))
            })
            .fold(
                NodeHierarchy::new(),
                |mut hierarchy, (node_id, node_hierarchy)| {
                    hierarchy.insert(node_id, node_hierarchy);
                    hierarchy
                },
            );

        let edges = outcome_node_edges(graph_info);
        let node_infos = node_infos(graph_info);

        InfoGraph::builder()
            .with_direction(GraphDir::Vertical)
            .with_hierarchy(hierarchy)
            .with_edges(edges)
            .with_node_infos(node_infos)
            .build()
    }
}

/// Returns a `NodeHierarchy` for the given node, if it has not already been
/// visited.
fn outcome_node_hierarchy(
    graph_info: &GraphInfo<ItemSpecInfo>,
    visited: &mut HashSet<FnId>,
    node_index: FnId,
) -> Option<NodeHierarchy> {
    if visited.contains(&node_index) {
        return None;
    }
    visited.insert(node_index);

    let mut hierarchy = NodeHierarchy::new();
    let children = graph_info.children(node_index);
    children
        .iter(&*graph_info)
        .filter_map(|(edge_index, child_node_index)| {
            // For outcome graphs, child nodes that:
            //
            // * are contained by parents nodes are represented as a nested node.
            // * reference data from parent nodes are represented by forward edges
            if matches!(
                graph_info.edge_weight(edge_index).copied(),
                Some(Edge::Contains)
            ) {
                Some(child_node_index)
            } else {
                None
            }
        })
        .for_each(|child_node_index| {
            if let Some(child_node_hierarchy) =
                outcome_node_hierarchy(graph_info, visited, child_node_index)
            {
                let item_spec_info = graph_info.node_weight(node_index).unwrap_or_else(|| {
                    panic!("`node_index`: `{node_index:?}` is invalid when accessing `flow_item_spec.graph_info`")
                });
                hierarchy.insert(
                    item_spec_info_to_node_id(item_spec_info),
                    child_node_hierarchy,
                );
            }
        });

    Some(hierarchy)
}

/// Returns the list of edges between items in the graph.
fn outcome_node_edges(graph_info: &GraphInfo<ItemSpecInfo>) -> IndexMap<EdgeId, [NodeId; 2]> {
    graph_info.iter_insertion_with_indices().fold(
        IndexMap::with_capacity(graph_info.node_count()),
        |mut edges, (node_index, item_spec_info)| {
            //
            let children = graph_info.children(node_index);
            children
                .iter(&*graph_info)
                .filter_map(|(edge_index, child_node_index)| {
                    // For outcome graphs, child nodes that:
                    //
                    // * are contained by parents nodes are represented as a nested node.
                    // * reference data from parent nodes are represented by forward edges
                    if matches!(
                        graph_info.edge_weight(edge_index).copied(),
                        Some(Edge::Logic)
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
                        EdgeId::try_from(format!("{child_item_id}__{item_id}")).expect(
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

/// Returns the list of edges between items in the graph for progress.
///
/// For progress graphs, an edge is rendered between pairs of predecessor and
/// successor items, regardless of whether their dependency is `Edge::Logic`
/// (adjacent) or `Edge::Contains` (nested).
fn progress_node_edges(graph_info: &GraphInfo<ItemSpecInfo>) -> IndexMap<EdgeId, [NodeId; 2]> {
    graph_info.iter_insertion_with_indices().fold(
        IndexMap::with_capacity(graph_info.node_count()),
        |mut edges, (node_index, item_spec_info)| {
            //
            let children = graph_info.children(node_index);
            children
                .iter(&*graph_info)
                .filter_map(|(edge_index, child_node_index)| {
                    //
                    // * are contained by parents nodes are represented as a nested node.
                    // * reference data from parent nodes are represented by forward edges
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
                        EdgeId::try_from(format!("{child_item_id}__{item_id}")).expect(
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
fn node_infos(graph_info: &GraphInfo<ItemSpecInfo>) -> IndexMap<NodeId, NodeInfo> {
    graph_info.iter_insertion_with_indices().fold(
        IndexMap::with_capacity(graph_info.node_count()),
        |mut node_infos, (_node_index, item_spec_info)| {
            let item_id = item_spec_info_to_node_id(item_spec_info);

            // Note: This does not have to be the ID, it can be a human readable name.
            let node_info = NodeInfo::new(item_id.to_string());

            node_infos.insert(item_id, node_info);

            node_infos
        },
    )
}

fn item_spec_info_to_node_id(item_spec_info: &ItemSpecInfo) -> NodeId {
    NodeId::try_from(item_spec_info.item_id.to_string())
        .expect("Expected `peace` `ItemId`s to be valid `dot_ix` `NodeId`s.`")
}
