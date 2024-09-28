use std::{collections::HashMap, str::FromStr};

use dot_ix_model::{
    common::{
        graphviz_attrs::EdgeDir, AnyId, EdgeId, Edges, GraphvizAttrs, NodeHierarchy, NodeId,
        NodeNames, TagId, TagNames,
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
use peace_webi_model::OutcomeInfoGraphVariant;
use smallvec::SmallVec;

/// Calculates the example / actual `InfoGraph` for a flow's outcome.
#[derive(Debug)]
pub struct OutcomeInfoGraphCalculator;

impl OutcomeInfoGraphCalculator {
    /// Returns the calculated `InfoGraph`.
    pub fn calculate<E>(
        flow: &Flow<E>,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        outcome_info_graph_variant: OutcomeInfoGraphVariant,
    ) -> InfoGraph
    where
        E: 'static,
    {
        let item_locations_and_interactions = match &outcome_info_graph_variant {
            OutcomeInfoGraphVariant::Example => {
                flow.item_locations_and_interactions_example(&params_specs, resources)
            }
            OutcomeInfoGraphVariant::Current { .. } => {
                flow.item_locations_and_interactions_current(&params_specs, resources)
            }
        };

        calculate_info_graph(
            flow,
            outcome_info_graph_variant,
            item_locations_and_interactions,
        )
    }
}

fn calculate_info_graph<E>(
    flow: &Flow<E>,
    outcome_info_graph_variant: OutcomeInfoGraphVariant,
    item_locations_and_interactions: ItemLocationsAndInteractions,
) -> InfoGraph
where
    E: 'static,
{
    let item_count = flow.graph().node_count();
    let ItemLocationsAndInteractions {
        item_location_trees,
        item_to_item_interactions,
        item_location_count,
    } = item_locations_and_interactions;

    let node_id_mappings_and_hierarchy =
        node_id_mappings_and_hierarchy(&item_location_trees, item_location_count);
    let NodeIdMappingsAndHierarchy {
        node_id_to_item_locations,
        mut item_location_to_node_id_segments,
        node_hierarchy,
    } = node_id_mappings_and_hierarchy;

    let node_names = node_id_to_item_locations.iter().fold(
        NodeNames::with_capacity(item_location_count),
        |mut node_names, (node_id, item_location)| {
            node_names.insert(node_id.clone(), item_location.name().to_string());
            node_names
        },
    );

    let tags = match &outcome_info_graph_variant {
        OutcomeInfoGraphVariant::Example => {
            let tags =
                flow.graph()
                    .iter()
                    .fold(TagNames::with_capacity(item_count), |mut tags, item| {
                        let tag_name = item.interactions_tag_name();

                        // For some reason taking away `.to_string()` causes an error to be
                        // highlighted on `flow.graph()`, rather than referring to `item.id()` as
                        // the cause of an extended borrow.
                        let tag_id = TagId::try_from(item.id().to_string())
                            .expect("Expected `tag_id` from `item_id` to be valid.");

                        tags.insert(tag_id, tag_name);

                        tags
                    });

            Some(tags)
        }
        OutcomeInfoGraphVariant::Current { .. } => None,
    };

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
    let item_interactions_processed_values = process_item_interactions(
        &outcome_info_graph_variant,
        item_location_count,
        &item_to_item_interactions,
        &node_id_to_item_locations,
        &mut item_location_to_node_id_segments,
    );
    let ItemInteractionsProcessedValues {
        edges,
        graphviz_attrs,
        mut theme,
    } = item_interactions_processed_values;

    theme_styles_augment(&item_location_trees, &node_id_to_item_locations, &mut theme);

    let mut info_graph = InfoGraph::default()
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
  100% { stroke-dashoffset: 198; }
}
@keyframes stroke-dashoffset-move-response {
  0%   { stroke-dashoffset: 0; }
  100% { stroke-dashoffset: -218; }
}
"#,
        ));

    if let Some(tags) = tags {
        info_graph = info_graph.with_tags(tags)
    }

    info_graph
}

/// Calculates edges and styles from `ItemInteraction`s.
///
/// # Code
///
/// Currently the code goes through the `ItemInteraction`s, and populates the
/// `Edges`, `Theme`, and `GraphvizAttrs`. This isn't as "clean" as iterating
/// over the `ItemInteraction`s per attribute that is to be computed, but
/// perhaps populating the different structures per `ItemInteraction` is more
/// manageable than remembering to update multiple functions.
fn process_item_interactions<'item_location>(
    outcome_info_graph_variant: &OutcomeInfoGraphVariant,
    item_location_count: usize,
    item_to_item_interactions: &'item_location IndexMap<peace_core::ItemId, Vec<ItemInteraction>>,
    node_id_to_item_locations: &IndexMap<NodeId, &'item_location ItemLocation>,
    item_location_to_node_id_segments: &mut HashMap<&'item_location ItemLocation, String>,
) -> ItemInteractionsProcessedValues {
    let item_interactions_processed_values = ItemInteractionsProcessedValues {
        edges: Edges::with_capacity(item_location_count),
        graphviz_attrs: GraphvizAttrs::new().with_edge_minlen_default(3),
        theme: Theme::new(),
    };
    let item_interactions_processed_values = item_to_item_interactions
        .iter()
        // The capacity could be worked out through the sum of all `ItemInteraction`s.
        //
        // For now we just use the `item_location_count` as a close approximation.
        .fold(
            item_interactions_processed_values,
            // Use `item_id` to compute `tags` and `tag_items`.
            |item_interactions_processed_values, (item_id, item_interactions)| {
                let ItemInteractionsProcessedValues {
                    mut edges,
                    mut graphviz_attrs,
                    mut theme,
                } = item_interactions_processed_values;

                item_interactions.iter().for_each(|item_interaction| {
                    let item_interactions_processing_ctx = ItemInteractionsProcessingCtx {
                        outcome_info_graph_variant,
                        node_id_to_item_locations,
                        item_location_to_node_id_segments,
                        edges: &mut edges,
                        theme: &mut theme,
                    };

                    match item_interaction {
                        ItemInteraction::Push(item_interaction_push) => {
                            process_item_interaction_push(
                                item_interactions_processing_ctx,
                                item_interaction_push,
                            );
                        }
                        ItemInteraction::Pull(item_interaction_pull) => {
                            process_item_interaction_pull(
                                item_interactions_processing_ctx,
                                &mut graphviz_attrs,
                                item_interaction_pull,
                            );
                        }
                        ItemInteraction::Within(item_interaction_within) => {
                            // TODO: compute theme
                        }
                    }
                });

                ItemInteractionsProcessedValues {
                    edges,
                    graphviz_attrs,
                    theme,
                }
            },
        );
    item_interactions_processed_values
}

/// Adds styles for nodes based on what kind of [`ItemLocation`] they represent.
fn theme_styles_augment(
    item_location_trees: &[ItemLocationTree],
    node_id_to_item_locations: &IndexMap<NodeId, &ItemLocation>,
    theme: &mut Theme,
) {
    // Use light styling for `ItemLocationType::Group` nodes.
    let mut css_class_partials_light = CssClassPartials::with_capacity(10);
    css_class_partials_light.insert(ThemeAttr::StrokeStyle, "dotted".to_string());
    css_class_partials_light.insert(ThemeAttr::StrokeShadeNormal, "300".to_string());
    css_class_partials_light.insert(ThemeAttr::StrokeShadeHover, "300".to_string());
    css_class_partials_light.insert(ThemeAttr::StrokeShadeFocus, "400".to_string());
    css_class_partials_light.insert(ThemeAttr::StrokeShadeActive, "500".to_string());
    css_class_partials_light.insert(ThemeAttr::FillShadeNormal, "50".to_string());
    css_class_partials_light.insert(ThemeAttr::FillShadeHover, "50".to_string());
    css_class_partials_light.insert(ThemeAttr::FillShadeFocus, "100".to_string());
    css_class_partials_light.insert(ThemeAttr::FillShadeActive, "200".to_string());

    node_id_to_item_locations
        .iter()
        .for_each(|(node_id, item_location)| {
            let css_class_partials = match item_location.r#type() {
                ItemLocationType::Host => {
                    // Specially colour some known hosts.
                    match item_location.name() {
                        ItemLocation::LOCALHOST => {
                            let mut css_class_partials = css_class_partials_light.clone();
                            css_class_partials.insert(ThemeAttr::ShapeColor, "blue".to_string());

                            Some(css_class_partials)
                        }
                        "github.com" => {
                            let mut css_class_partials = css_class_partials_light.clone();
                            css_class_partials.insert(ThemeAttr::ShapeColor, "purple".to_string());

                            Some(css_class_partials)
                        }
                        _ => {
                            // Not all hosts should be styled light -- only the ones that are top
                            // level. i.e. if the host is inside a group, then it should likely be
                            // styled darker.
                            if item_location_trees
                                .iter()
                                .map(ItemLocationTree::item_location)
                                .find(|item_location_top_level| {
                                    item_location_top_level == item_location
                                })
                                .is_some()
                            {
                                Some(css_class_partials_light.clone())
                            } else {
                                None
                            }
                        }
                    }
                }
                ItemLocationType::Group => Some(css_class_partials_light.clone()),
                _ => None,
            };

            if let Some(css_class_partials) = css_class_partials {
                theme.styles.insert(
                    AnyIdOrDefaults::AnyId(AnyId::from(node_id.clone())),
                    css_class_partials,
                );
            }
        });
}

/// Inserts an edge between the `from` and `to` nodes of an
/// [`ItemInteractionPush`].
fn process_item_interaction_push<'f, 'item_location>(
    item_interactions_processing_ctx: ItemInteractionsProcessingCtx<'f, 'item_location>,
    item_interaction_push: &'item_location ItemInteractionPush,
) {
    let ItemInteractionsProcessingCtx {
        outcome_info_graph_variant,
        node_id_to_item_locations,
        item_location_to_node_id_segments,
        edges,
        theme,
    } = item_interactions_processing_ctx;
    // Use the outermost `ItemLocationType::Host` node.
    // The `NodeId` for the item location is the longest node ID that contains all
    // of the `node_id_segment`s of the selected item location's ancestors.
    let node_id_from = {
        let item_location_ancestors_iter = || {
            let mut host_found = false;
            let mut location_from_iter = item_interaction_push.location_from().iter();
            std::iter::from_fn(move || {
                if host_found {
                    return None;
                }

                let item_location = location_from_iter.next();
                if let Some(item_location) = item_location.as_ref() {
                    host_found = item_location.r#type() == ItemLocationType::Host;
                }
                item_location
            })
            .fuse()
        };

        let node_id_from = node_id_from_item_location(
            item_location_to_node_id_segments,
            item_location_ancestors_iter,
        );

        node_id_with_ancestor_find(node_id_to_item_locations, node_id_from)
    };

    // Use the innermost node.
    let node_id_to = {
        let node_id_to = node_id_from_item_location(item_location_to_node_id_segments, || {
            item_interaction_push.location_to().iter()
        });

        node_id_with_ancestor_find(node_id_to_item_locations, node_id_to)
    };

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
}

/// Inserts an edge between the `client` and `server` nodes of an
/// [`ItemInteractionPull`].
fn process_item_interaction_pull<'f, 'item_location>(
    item_interactions_processing_ctx: ItemInteractionsProcessingCtx<'f, 'item_location>,
    graphviz_attrs: &mut GraphvizAttrs,
    item_interaction_pull: &'item_location ItemInteractionPull,
) {
    let ItemInteractionsProcessingCtx {
        outcome_info_graph_variant,
        node_id_to_item_locations,
        item_location_to_node_id_segments,
        edges,
        theme,
    } = item_interactions_processing_ctx;

    // Use the outermost `ItemLocationType::Host` node.
    let node_id_client = {
        let item_location_ancestors_iter = || {
            let mut host_found = false;
            let mut location_from_iter = item_interaction_pull.location_client().iter();
            std::iter::from_fn(move || {
                if host_found {
                    return None;
                }

                let item_location = location_from_iter.next();
                if let Some(item_location) = item_location.as_ref() {
                    host_found = item_location.r#type() == ItemLocationType::Host;
                }
                item_location
            })
            .fuse()
        };

        let node_id_client = node_id_from_item_location(
            item_location_to_node_id_segments,
            item_location_ancestors_iter,
        );

        node_id_with_ancestor_find(node_id_to_item_locations, node_id_client)
    };

    // Use the innermost node.
    let node_id_server = {
        let node_id_server = node_id_from_item_location(item_location_to_node_id_segments, || {
            item_interaction_pull.location_server().iter()
        });

        node_id_with_ancestor_find(node_id_to_item_locations, node_id_server)
    };

    let edge_id_request =
        EdgeId::from_str(&format!("{node_id_client}___{node_id_server}___request"))
            .expect("Expected edge ID from item location ID to be valid for `edge_id_request`.");
    edges.insert(
        edge_id_request.clone(),
        [node_id_server.clone(), node_id_client.clone()],
    );

    let edge_id_response =
        EdgeId::from_str(&format!("{node_id_client}___{node_id_server}___response"))
            .expect("Expected edge ID from item location ID to be valid for `edge_id_response`.");
    edges.insert(
        edge_id_response.clone(),
        [node_id_server.clone(), node_id_client.clone()],
    );

    graphviz_attrs
        .edge_dirs
        .insert(edge_id_request.clone(), EdgeDir::Back);

    let mut css_class_partials_request = CssClassPartials::with_capacity(6);
    css_class_partials_request.insert(
        ThemeAttr::Animate,
        "[stroke-dashoffset-move-request_1.5s_linear_infinite]".to_string(),
    );
    css_class_partials_request.insert(ThemeAttr::ShapeColor, "blue".to_string());
    css_class_partials_request.insert(
        ThemeAttr::StrokeStyle,
        "dasharray:0,50,12,2,4,2,2,2,1,2,1,120".to_string(),
    );
    css_class_partials_request.insert(ThemeAttr::StrokeWidth, "[1px]".to_string());
    css_class_partials_request.insert(ThemeAttr::StrokeShadeNormal, "600".to_string());
    css_class_partials_request.insert(ThemeAttr::FillShadeNormal, "500".to_string());
    theme.styles.insert(
        AnyIdOrDefaults::AnyId(AnyId::from(edge_id_request)),
        css_class_partials_request,
    );

    let mut css_class_partials_response = CssClassPartials::with_capacity(6);
    css_class_partials_response.insert(
        ThemeAttr::Animate,
        "[stroke-dashoffset-move-response_1.5s_linear_infinite]".to_string(),
    );
    css_class_partials_response.insert(ThemeAttr::ShapeColor, "blue".to_string());
    css_class_partials_response.insert(
        ThemeAttr::StrokeStyle,
        "dasharray:0,120,1,2,1,2,2,2,4,2,8,2,20,50".to_string(),
    );
    css_class_partials_response.insert(ThemeAttr::StrokeWidth, "[2px]".to_string());
    css_class_partials_response.insert(ThemeAttr::StrokeShadeNormal, "600".to_string());
    css_class_partials_response.insert(ThemeAttr::FillShadeNormal, "500".to_string());
    theme.styles.insert(
        AnyIdOrDefaults::AnyId(AnyId::from(edge_id_response)),
        css_class_partials_response,
    );
}

/// Returns the node ID that ends with the calculated node ID, in case another
/// `Item` has provided an ancestor as context.
///
/// Not sure if we need to find the longest node ID (which incurs one more
/// sort), but the current implementation just returns the first match.
fn node_id_with_ancestor_find(
    node_id_to_item_locations: &IndexMap<NodeId, &ItemLocation>,
    node_id_from: NodeId,
) -> NodeId {
    node_id_to_item_locations
        .keys()
        .find(|node_id| node_id.ends_with(node_id_from.as_str()))
        .cloned()
        .unwrap_or(node_id_from)
}

/// Returns a map of `NodeId` to the `ItemLocation` it is associated with, and
/// the `NodeHierarchy` constructed from the `ItemLocationTree`s.
fn node_id_mappings_and_hierarchy<'item_location>(
    item_location_trees: &'item_location [ItemLocationTree],
    item_location_count: usize,
) -> NodeIdMappingsAndHierarchy<'item_location> {
    let node_id_mappings_and_hierarchy = NodeIdMappingsAndHierarchy {
        node_id_to_item_locations: IndexMap::with_capacity(item_location_count),
        item_location_to_node_id_segments: HashMap::with_capacity(item_location_count),
        node_hierarchy: NodeHierarchy::with_capacity(item_location_trees.len()),
    };

    item_location_trees.iter().fold(
        node_id_mappings_and_hierarchy,
        |mut node_id_mappings_and_hierarchy, item_location_tree| {
            let NodeIdMappingsAndHierarchy {
                node_id_to_item_locations,
                item_location_to_node_id_segments,
                node_hierarchy,
            } = &mut node_id_mappings_and_hierarchy;

            let item_location = item_location_tree.item_location();

            // Probably won't go more than 8 deep.
            let mut item_location_ancestors = SmallVec::<[&ItemLocation; 8]>::new();
            item_location_ancestors.push(item_location);

            let node_id = node_id_from_item_location(item_location_to_node_id_segments, || {
                item_location_ancestors.clone().into_iter()
            });

            node_id_to_item_locations.insert(node_id.clone(), item_location);

            let node_hierarchy_top_level = node_hierarchy_build_and_item_location_insert(
                item_location_tree,
                node_id_to_item_locations,
                item_location_to_node_id_segments,
                item_location_ancestors,
            );
            node_hierarchy.insert(node_id, node_hierarchy_top_level);

            node_id_mappings_and_hierarchy
        },
    )
}

/// Returns the [`NodeId`] for the given [`ItemLocation`].
///
/// This is computed from all of the node ID segments from all of the node's
/// ancestors.
fn node_id_from_item_location<'item_location, F, I>(
    item_location_to_node_id_segments: &mut HashMap<&'item_location ItemLocation, String>,
    item_location_ancestors_iter_fn: F,
) -> NodeId
where
    F: Fn() -> I,
    I: Iterator<Item = &'item_location ItemLocation>,
{
    let item_location_ancestors_iter_for_capacity = item_location_ancestors_iter_fn();
    let capacity = item_location_ancestors_iter_for_capacity.fold(
        0usize,
        |capacity_acc, item_location_ancestor| {
            let node_id_segment = item_location_to_node_id_segments
                .entry(item_location_ancestor)
                .or_insert_with(move || node_id_segment_from_item_location(item_location_ancestor));

            capacity_acc + node_id_segment.len() + 3
        },
    );
    let mut node_id = item_location_ancestors_iter_fn()
        .filter_map(|item_location_ancestor| {
            item_location_to_node_id_segments.get(item_location_ancestor)
        })
        .fold(
            String::with_capacity(capacity),
            |mut node_id_buffer, node_id_segment| {
                node_id_buffer.push_str(&node_id_segment);
                node_id_buffer.push_str("___");
                node_id_buffer
            },
        );

    node_id.truncate(node_id.len() - "___".len());

    let node_id =
        NodeId::try_from(node_id).expect("Expected node ID from item location ID to be valid.");
    node_id
}

/// Returns a `&str` segment that can be used as part of the `NodeId` for the
/// given [`ItemLocation`].
///
/// An [`ItemLocation`]'s [`NodeId`] needs to be joined with the parent segments
/// from its ancestors, otherwise two different `path__path_to_file`
/// [`ItemLocation`]s may be accidentally merged.
fn node_id_segment_from_item_location(item_location: &ItemLocation) -> String {
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

    format!("{item_location_type}___{name_transformed}")
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
    item_location_to_node_id_segments: &mut HashMap<&'item_location ItemLocation, String>,
    item_location_ancestors: SmallVec<[&'item_location ItemLocation; 8]>,
) -> NodeHierarchy {
    let mut node_hierarchy = NodeHierarchy::with_capacity(item_location_tree.children().len());

    item_location_tree
        .children()
        .iter()
        .for_each(|child_item_location_tree| {
            let child_item_location = child_item_location_tree.item_location();
            let mut child_item_location_ancestors = item_location_ancestors.clone();
            child_item_location_ancestors.push(child_item_location);

            let child_node_id =
                node_id_from_item_location(item_location_to_node_id_segments, || {
                    child_item_location_ancestors.clone().into_iter()
                });
            node_id_to_item_locations.insert(child_node_id.clone(), child_item_location);

            let child_hierarchy = node_hierarchy_build_and_item_location_insert(
                child_item_location_tree,
                node_id_to_item_locations,
                item_location_to_node_id_segments,
                child_item_location_ancestors,
            );
            node_hierarchy.insert(child_node_id, child_hierarchy);
        });

    node_hierarchy
}

struct NodeIdMappingsAndHierarchy<'item_location> {
    node_id_to_item_locations: IndexMap<NodeId, &'item_location ItemLocation>,
    item_location_to_node_id_segments: HashMap<&'item_location ItemLocation, String>,
    node_hierarchy: NodeHierarchy,
}

struct ItemInteractionsProcessingCtx<'f, 'item_location> {
    outcome_info_graph_variant: &'f OutcomeInfoGraphVariant,
    node_id_to_item_locations: &'f IndexMap<NodeId, &'item_location ItemLocation>,
    item_location_to_node_id_segments: &'f mut HashMap<&'item_location ItemLocation, String>,
    edges: &'f mut Edges,
    theme: &'f mut Theme,
}

struct ItemInteractionsProcessedValues {
    edges: Edges,
    graphviz_attrs: GraphvizAttrs,
    theme: Theme,
}
