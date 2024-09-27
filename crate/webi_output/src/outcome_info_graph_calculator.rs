use std::str::FromStr;

use dot_ix_model::{
    common::{
        graphviz_attrs::EdgeDir, AnyId, EdgeId, Edges, GraphvizAttrs, NodeHierarchy, NodeId,
        NodeNames,
    },
    info_graph::{GraphDir, InfoGraph},
    theme::{AnyIdOrDefaults, CssClassPartials, Theme, ThemeAttr},
};
use indexmap::IndexMap;
use peace_item_model::{
    ItemInteraction, ItemInteractionPull, ItemInteractionPush, ItemLocation, ItemLocationTree,
    ItemLocationType, ItemLocationsAndInteractions,
};
use peace_params::ParamsSpecs;
use peace_resource_rt::{resources::ts::SetUp, Resources};
use peace_rt_model::Flow;

/// Calculates the example / actual `InfoGraph` for a flow's outcome.
#[derive(Debug)]
pub struct OutcomeInfoGraphCalculator;

impl OutcomeInfoGraphCalculator {
    /// Returns the `InfoGraph` calculated using example state.
    pub fn calculate_example<E>(
        flow: &Flow<E>,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> InfoGraph
    where
        E: 'static,
    {
        let item_locations_and_interactions =
            flow.item_locations_and_interactions_example(&params_specs, resources);

        calculate_info_graph(item_locations_and_interactions)
    }

    /// Returns the `InfoGraph` calculated using example state.
    pub fn calculate_current<E>(
        flow: &Flow<E>,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> InfoGraph
    where
        E: 'static,
    {
        let item_locations_and_interactions =
            flow.item_locations_and_interactions_current(&params_specs, resources);

        calculate_info_graph(item_locations_and_interactions)
    }
}

fn calculate_info_graph(
    item_locations_and_interactions: ItemLocationsAndInteractions,
) -> InfoGraph {
    let ItemLocationsAndInteractions {
        item_location_trees,
        item_to_item_interactions,
        item_location_count,
    } = item_locations_and_interactions;

    let node_id_mappings_and_hierarchy =
        node_id_mappings_and_hierarchy(&item_location_trees, item_location_count);
    let NodeIdMappingsAndHierarchy {
        node_id_to_item_locations,
        item_location_to_node_ids,
        node_hierarchy,
    } = node_id_mappings_and_hierarchy;

    let node_names = node_id_to_item_locations.iter().fold(
        NodeNames::with_capacity(item_location_count),
        |mut node_names, (node_id, item_location)| {
            node_names.insert(node_id.clone(), item_location.name().to_string());
            node_names
        },
    );

    // 1. Each item interaction knows the `ItemLocation`s
    // 2. We need to be able to translate from an `ItemLocation`, to the `NodeId`s
    //    that we need to link as edges.
    // 3. We have a way to map from `ItemLocation` to `NodeId` using the
    //    `node_id_from_item_location` function.
    // 4. So, either we calculate the `NodeId` from each `ItemLocation` in each
    //    interaction again, or `ItemLocation` must implement `Hash` and `Eq`, and
    //    look it up.
    // 5. It already implements `Hash` and `Eq`, so let's construct a
    //    `Map<ItemLocation, NodeId>`.
    // 6. Then we can iterate through `item_to_item_interactions`, and for each
    //    `ItemLocation`, look up the map from 5, and add an edge.
    let (edges, graphviz_attrs, theme) = item_to_item_interactions
        .iter()
        // The capacity could be worked out through the sum of all `ItemInteraction`s.
        //
        // For now we just use the `item_location_count` as a close approximation.
        .fold(
            (
                Edges::with_capacity(item_location_count),
                GraphvizAttrs::new().with_edge_minlen_default(3),
                Theme::new(),
            ),
            // TODO: Use `item_id` to compute `tags` and `tag_items`.
            |(mut edges, mut graphviz_attrs, mut theme), (item_id, item_interactions)| {
                item_interactions
                    .iter()
                    .for_each(|item_interaction| match item_interaction {
                        ItemInteraction::Push(item_interaction_push) => {
                            process_item_interaction_push(
                                &item_location_to_node_ids,
                                &mut edges,
                                &mut theme,
                                item_interaction_push,
                            );
                        }
                        ItemInteraction::Pull(item_interaction_pull) => {
                            process_item_interaction_pull(
                                &item_location_to_node_ids,
                                &mut edges,
                                &mut theme,
                                &mut graphviz_attrs,
                                item_interaction_pull,
                            );
                        }
                        ItemInteraction::Within(item_interaction_within) => {
                            // TODO: compute theme
                        }
                    });

                (edges, graphviz_attrs, theme)
            },
        );

    InfoGraph::default()
        .with_direction(GraphDir::Vertical)
        .with_hierarchy(node_hierarchy)
        .with_node_names(node_names)
        .with_edges(edges)
        .with_graphviz_attrs(graphviz_attrs)
        .with_theme(theme)
        .with_css(String::from(
            r#"
@keyframes stroke-dashoffset-move {
  0%   { stroke-dashoffset: 136; }
  100% { stroke-dashoffset: 0; }
}
@keyframes stroke-dashoffset-move-request {
  0%   { stroke-dashoffset: 0; }
  100% { stroke-dashoffset: 228; }
}
@keyframes stroke-dashoffset-move-response {
  0%   { stroke-dashoffset: 0; }
  100% { stroke-dashoffset: -248; }
}
"#,
        ))
}

/// Inserts an edge between the `from` and `to` nodes of an
/// [`ItemInteractionPush`].
fn process_item_interaction_push(
    item_location_to_node_ids: &IndexMap<&ItemLocation, NodeId>,
    edges: &mut Edges,
    theme: &mut Theme,
    item_interaction_push: &ItemInteractionPush,
) {
    // Use the outermost `ItemLocationType::Host` node.
    let node_id_from = item_interaction_push
        .location_from()
        .iter()
        .find(|item_location| item_location.r#type() == ItemLocationType::Host)
        .or_else(|| item_interaction_push.location_from().iter().next())
        .and_then(|item_location| item_location_to_node_ids.get(item_location));

    // Use the innermost `ItemLocationType::Path` node.
    let node_id_to = item_interaction_push
        .location_to()
        .iter()
        .rev()
        .find(|item_location| item_location.r#type() == ItemLocationType::Path)
        .or_else(|| item_interaction_push.location_to().iter().next())
        .and_then(|item_location| item_location_to_node_ids.get(item_location));

    if let Some((node_id_from, node_id_to)) = node_id_from.zip(node_id_to) {
        let edge_id = EdgeId::from_str(&format!("{node_id_from}___{node_id_to}"))
            .expect("Expected edge ID from item location ID to be valid for `edge_id`.");
        edges.insert(edge_id.clone(), [node_id_from.clone(), node_id_to.clone()]);

        let mut css_class_partials = CssClassPartials::with_capacity(5);
        css_class_partials.insert(
            ThemeAttr::Animate,
            "[stroke-dashoffset-move_1s_linear_infinite]".to_string(),
        );
        css_class_partials.insert(ThemeAttr::ShapeColor, "blue".to_string());
        css_class_partials.insert(
            ThemeAttr::StrokeStyle,
            "dasharray:0,40,1,2,1,2,2,2,4,2,8,2,20,50".to_string(),
        );
        css_class_partials.insert(ThemeAttr::StrokeShadeNormal, "600".to_string());
        css_class_partials.insert(ThemeAttr::FillShadeNormal, "500".to_string());

        theme.styles.insert(
            AnyIdOrDefaults::AnyId(AnyId::from(edge_id)),
            css_class_partials,
        );
    } else {
        // One of the `ItemLocationAncestors` was empty, which should be rare.
    }
}

/// Inserts an edge between the `client` and `server` nodes of an
/// [`ItemInteractionPull`].
fn process_item_interaction_pull(
    item_location_to_node_ids: &IndexMap<&ItemLocation, NodeId>,
    edges: &mut Edges,
    theme: &mut Theme,
    graphviz_attrs: &mut GraphvizAttrs,
    item_interaction_pull: &ItemInteractionPull,
) {
    // Use the outermost `ItemLocationType::Host` node.
    let node_id_client = item_interaction_pull
        .location_client()
        .iter()
        .find(|item_location| item_location.r#type() == ItemLocationType::Host)
        .or_else(|| item_interaction_pull.location_client().iter().next())
        .and_then(|item_location| item_location_to_node_ids.get(item_location));

    // Use the innermost `ItemLocationType::Path` node.
    let node_id_server = item_interaction_pull
        .location_server()
        .iter()
        .rev()
        .find(|item_location| item_location.r#type() == ItemLocationType::Path)
        .or_else(|| item_interaction_pull.location_server().iter().next())
        .and_then(|item_location| item_location_to_node_ids.get(item_location));

    if let Some((node_id_client, node_id_server)) = node_id_client.zip(node_id_server) {
        let edge_id_request = EdgeId::from_str(&format!(
            "{node_id_client}___{node_id_server}___request"
        ))
        .expect("Expected edge ID from item location ID to be valid for `edge_id_request`.");
        edges.insert(
            edge_id_request.clone(),
            [node_id_server.clone(), node_id_client.clone()],
        );

        let edge_id_response = EdgeId::from_str(&format!(
            "{node_id_client}___{node_id_server}___response"
        ))
        .expect("Expected edge ID from item location ID to be valid for `edge_id_response`.");
        edges.insert(
            edge_id_response.clone(),
            [node_id_server.clone(), node_id_client.clone()],
        );

        graphviz_attrs
            .edge_dirs
            .insert(edge_id_request.clone(), EdgeDir::Back);

        let mut css_class_partials_request = CssClassPartials::with_capacity(4);
        css_class_partials_request.insert(
            ThemeAttr::Animate,
            "[stroke-dashoffset-move-request_2s_linear_infinite]".to_string(),
        );
        css_class_partials_request.insert(ThemeAttr::ShapeColor, "blue".to_string());
        css_class_partials_request.insert(
            ThemeAttr::StrokeStyle,
            "dasharray:0,120,1,2,1,2,2,2,4,2,8,2,20,80".to_string(),
        );
        css_class_partials_request.insert(ThemeAttr::StrokeWidth, "[1px]".to_string());
        theme.styles.insert(
            AnyIdOrDefaults::AnyId(AnyId::from(edge_id_request)),
            css_class_partials_request,
        );

        let mut css_class_partials_response = CssClassPartials::with_capacity(4);
        css_class_partials_response.insert(
            ThemeAttr::Animate,
            "[stroke-dashoffset-move-response_2s_linear_infinite]".to_string(),
        );
        css_class_partials_response.insert(ThemeAttr::ShapeColor, "blue".to_string());
        css_class_partials_response.insert(
            ThemeAttr::StrokeStyle,
            "dasharray:0,120,1,2,1,2,2,2,4,2,8,2,20,80".to_string(),
        );
        css_class_partials_response.insert(ThemeAttr::StrokeWidth, "[2px]".to_string());
        theme.styles.insert(
            AnyIdOrDefaults::AnyId(AnyId::from(edge_id_response)),
            css_class_partials_response,
        );
    } else {
        // One of the `ItemLocationAncestors` was empty, which should be rare.
    }
}

/// Returns a map of `NodeId` to the `ItemLocation` it is associated with, and
/// the `NodeHierarchy` constructed from the `ItemLocationTree`s.
fn node_id_mappings_and_hierarchy<'item_location>(
    item_location_trees: &'item_location [ItemLocationTree],
    item_location_count: usize,
) -> NodeIdMappingsAndHierarchy<'item_location> {
    let node_id_mappings_and_hierarchy = NodeIdMappingsAndHierarchy {
        node_id_to_item_locations: IndexMap::with_capacity(item_location_count),
        item_location_to_node_ids: IndexMap::with_capacity(item_location_count),
        node_hierarchy: NodeHierarchy::with_capacity(item_location_trees.len()),
    };
    item_location_trees.iter().fold(
        node_id_mappings_and_hierarchy,
        |mut node_id_mappings_and_hierarchy, item_location_tree| {
            let NodeIdMappingsAndHierarchy {
                node_id_to_item_locations,
                item_location_to_node_ids,
                node_hierarchy,
            } = &mut node_id_mappings_and_hierarchy;

            let item_location = item_location_tree.item_location();
            let node_id = node_id_from_item_location(item_location);

            node_id_to_item_locations.insert(node_id.clone(), item_location);
            item_location_to_node_ids.insert(item_location, node_id.clone());

            let node_hierarchy_top_level = node_hierarchy_build_and_item_location_insert(
                item_location_tree,
                node_id_to_item_locations,
                item_location_to_node_ids,
            );
            node_hierarchy.insert(node_id, node_hierarchy_top_level);

            node_id_mappings_and_hierarchy
        },
    )
}

fn node_id_from_item_location(item_location: &ItemLocation) -> NodeId {
    let item_location_type = match item_location.r#type() {
        ItemLocationType::Group => "group",
        ItemLocationType::Host => "host",
        ItemLocationType::Path => "path",
    };
    let name = item_location.name();
    let name_transformed =
        name.chars()
            .fold(String::with_capacity(name.len()), |mut name_acc, c| {
                match c {
                    'a'..='z' | '0'..='9' => name_acc.push(c),
                    'A'..='Z' => c.to_lowercase().for_each(|c| name_acc.push(c)),
                    _ => name_acc.push_str("__"),
                }
                name_acc
            });
    let node_id = NodeId::try_from(format!("{item_location_type}___{name_transformed}"))
        .expect("Expected node ID from item location ID to be valid.");
    node_id
}

/// Recursively constructs the `NodeHierarchy` and populates a map to facilitate
/// calculation of `InfoGraph` representing `ItemLocation`s.
///
/// Each `Node` corresponds to one `ItemLocation`.
///
/// Because:
///
/// * Each `ItemInteraction` can include multiple `ItemLocation`s -- both nested
///   and separate, and
/// * We need to style each node
///
/// it is useful to be able to retrieve the `ItemLocation` for each `Node` we
/// are adding attributes for.
fn node_hierarchy_build_and_item_location_insert<'item_location>(
    item_location_tree: &'item_location ItemLocationTree,
    node_id_to_item_locations: &mut IndexMap<NodeId, &'item_location ItemLocation>,
    item_location_to_node_ids: &mut IndexMap<&'item_location ItemLocation, NodeId>,
) -> NodeHierarchy {
    let mut node_hierarchy = NodeHierarchy::with_capacity(item_location_tree.children().len());

    item_location_tree
        .children()
        .iter()
        .for_each(|child_item_location_tree| {
            let child_item_location = child_item_location_tree.item_location();
            let child_node_id = node_id_from_item_location(child_item_location);
            node_id_to_item_locations.insert(child_node_id.clone(), child_item_location);
            item_location_to_node_ids.insert(child_item_location, child_node_id.clone());

            let child_hierarchy = node_hierarchy_build_and_item_location_insert(
                child_item_location_tree,
                node_id_to_item_locations,
                item_location_to_node_ids,
            );
            node_hierarchy.insert(child_node_id, child_hierarchy);
        });

    node_hierarchy
}

struct NodeIdMappingsAndHierarchy<'item_location> {
    node_id_to_item_locations: IndexMap<NodeId, &'item_location ItemLocation>,
    item_location_to_node_ids: IndexMap<&'item_location ItemLocation, NodeId>,
    node_hierarchy: NodeHierarchy,
}
