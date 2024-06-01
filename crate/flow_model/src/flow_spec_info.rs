use std::collections::HashSet;

use dot_ix::model::{
    common::{EdgeId, Edges, NodeHierarchy, NodeId, NodeNames},
    info_graph::{GraphDir, InfoGraph},
};
use fn_graph::{daggy::Walker, Edge, FnId, GraphInfo};
use peace_core::FlowId;

use serde::{Deserialize, Serialize};

use crate::StepSpecInfo;

/// Serializable representation of how a [`Flow`] is configured.
///
/// [`Flow`]: https://docs.rs/peace_rt_model/latest/peace_rt_model/struct.Flow.html
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FlowSpecInfo {
    /// ID of the flow.
    pub flow_id: FlowId,
    /// Serialized representation of the flow graph.
    pub graph_info: GraphInfo<StepSpecInfo>,
}

impl FlowSpecInfo {
    /// Returns a new `FlowSpecInfo`.
    pub fn new(flow_id: FlowId, graph_info: GraphInfo<StepSpecInfo>) -> Self {
        Self {
            flow_id,
            graph_info,
        }
    }

    /// Returns an [`InfoGraph`] that represents the progress of the flow's
    /// execution.
    pub fn to_progress_info_graph(&self) -> InfoGraph {
        let graph_info = &self.graph_info;
        let step_count = graph_info.node_count();

        let hierarchy = graph_info.iter_insertion_with_indices().fold(
            NodeHierarchy::with_capacity(step_count),
            |mut hierarchy, (_node_index, step_spec_info)| {
                let node_id = step_spec_info_to_node_id(step_spec_info);
                // Progress nodes have no nested nodes.
                hierarchy.insert(node_id, NodeHierarchy::new());
                hierarchy
            },
        );

        let edges = progress_node_edges(graph_info);
        let node_names = node_names(graph_info);

        InfoGraph::builder()
            .with_direction(GraphDir::Vertical)
            .with_hierarchy(hierarchy)
            .with_edges(edges)
            .with_node_names(node_names)
            .build()
    }

    /// Returns an [`InfoGraph`] that represents the outcome of the flow's
    /// execution.
    pub fn to_outcome_info_graph(&self) -> InfoGraph {
        let graph_info = &self.graph_info;
        let step_count = graph_info.node_count();

        let mut visited = HashSet::with_capacity(step_count);
        let visited = &mut visited;
        let hierarchy = graph_info
            .iter_insertion_with_indices()
            .filter_map(|(node_index, step_spec_info)| {
                let node_hierarchy = outcome_node_hierarchy(graph_info, visited, node_index);
                let node_id = step_spec_info_to_node_id(step_spec_info);
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
        let node_names = node_names(graph_info);

        InfoGraph::builder()
            .with_direction(GraphDir::Vertical)
            .with_hierarchy(hierarchy)
            .with_edges(edges)
            .with_node_names(node_names)
            .build()
    }
}

/// Returns a `NodeHierarchy` for the given node, if it has not already been
/// visited.
fn outcome_node_hierarchy(
    graph_info: &GraphInfo<StepSpecInfo>,
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
        .iter(graph_info)
        .filter_map(|(edge_index, child_node_index)| {
            // For outcome graphs, child nodes that:
            //
            // * are contained by parents nodes are represented as a nested node.
            // * reference data from parent nodes are represented by forward edges.
            //
            // We actually want to determine nesting from the following information:
            //
            // * Host machines:
            //
            //     A file transfer would have a source host, source path, dest host, dest
            //     path, and these exist in the same Step's parameters.
            //
            // * Physical nesting:
            //
            //     - Configuration that lives inside a server.
            //     - Cloud resource that lives inside a subnet.
            //
            //     Should this be provided by the step or tool developer?
            //
            //     Probably the step. The step knows its parameters (which may be mapped
            //     from other steps' state), so the containment is returned based on the
            //     step knowing its parent container is from a source / destination
            //     parameter.
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
                let step_spec_info = &graph_info[child_node_index];
                hierarchy.insert(
                    step_spec_info_to_node_id(step_spec_info),
                    child_node_hierarchy,
                );
            }
        });

    Some(hierarchy)
}

/// Returns the list of edges between steps in the graph.
fn outcome_node_edges(graph_info: &GraphInfo<StepSpecInfo>) -> Edges {
    graph_info.iter_insertion_with_indices().fold(
        Edges::with_capacity(graph_info.node_count()),
        |mut edges, (node_index, step_spec_info)| {
            //
            let children = graph_info.children(node_index);
            children
                .iter(graph_info)
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
                    let step_id = step_spec_info_to_node_id(step_spec_info);
                    let child_step_id = step_spec_info_to_node_id(&graph_info[child_node_index]);
                    edges.insert(
                        EdgeId::try_from(format!("{step_id}__{child_step_id}")).expect(
                            "Expected `peace` `StepId`s concatenated \
                            to be valid `dot_ix` `EdgeId`s.",
                        ),
                        [step_id, child_step_id],
                    );
                });

            edges
        },
    )
}

/// Returns the list of edges between steps in the graph for progress.
///
/// For progress graphs, an edge is rendered between pairs of predecessor and
/// successor steps, regardless of whether their dependency is `Edge::Logic`
/// (adjacent) or `Edge::Contains` (nested).
fn progress_node_edges(graph_info: &GraphInfo<StepSpecInfo>) -> Edges {
    graph_info.iter_insertion_with_indices().fold(
        Edges::with_capacity(graph_info.node_count()),
        |mut edges, (node_index, step_spec_info)| {
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
                    let step_id = step_spec_info_to_node_id(step_spec_info);
                    let child_step_id = step_spec_info_to_node_id(&graph_info[child_node_index]);
                    edges.insert(
                        EdgeId::try_from(format!("{step_id}__{child_step_id}")).expect(
                            "Expected `peace` `StepId`s concatenated \
                            to be valid `dot_ix` `EdgeId`s.",
                        ),
                        [step_id, child_step_id],
                    );
                });

            edges
        },
    )
}

/// Returns the list of edges between steps in the graph.
fn node_names(graph_info: &GraphInfo<StepSpecInfo>) -> NodeNames {
    graph_info.iter_insertion_with_indices().fold(
        NodeNames::with_capacity(graph_info.node_count()),
        |mut node_names, (_node_index, step_spec_info)| {
            let step_id = step_spec_info_to_node_id(step_spec_info);

            // Note: This does not have to be the ID, it can be a human readable name.
            let node_name = step_id.to_string();

            node_names.insert(step_id, node_name);

            node_names
        },
    )
}

fn step_spec_info_to_node_id(step_spec_info: &StepSpecInfo) -> NodeId {
    NodeId::try_from(step_spec_info.step_id.to_string())
        .expect("Expected `peace` `StepId`s to be valid `dot_ix` `NodeId`s.`")
}
