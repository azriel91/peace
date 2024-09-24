use dot_ix_model::{
    common::{NodeHierarchy, NodeId, NodeNames},
    info_graph::InfoGraph,
};
use indexmap::IndexMap;
use peace_item_model::{
    ItemLocation, ItemLocationTree, ItemLocationType, ItemLocationsAndInteractions,
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

        // TODO: add edges
        item_to_item_interactions,
        item_location_count,
    } = item_locations_and_interactions;

    let (node_id_to_item_locations, node_hierarchy) = item_location_trees.iter().fold(
        (
            IndexMap::<NodeId, &ItemLocation>::with_capacity(item_location_count),
            NodeHierarchy::with_capacity(item_location_trees.len()),
        ),
        |(mut node_id_to_item_locations, mut node_hierarchy_all), item_location_tree| {
            let item_location = item_location_tree.item_location();
            let node_id = node_id_from_item_location(item_location);

            node_id_to_item_locations.insert(node_id.clone(), item_location);

            let node_hierarchy_top_level = node_hierarchy_build_and_item_location_insert(
                item_location_tree,
                &mut node_id_to_item_locations,
            );
            node_hierarchy_all.insert(node_id, node_hierarchy_top_level);

            (node_id_to_item_locations, node_hierarchy_all)
        },
    );

    let node_names = node_id_to_item_locations.iter().fold(
        NodeNames::with_capacity(item_location_count),
        |mut node_names, (node_id, item_location)| {
            node_names.insert(node_id.clone(), item_location.name().to_string());
            node_names
        },
    );

    InfoGraph::default()
        .with_hierarchy(node_hierarchy)
        .with_node_names(node_names)
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
) -> NodeHierarchy {
    let mut node_hierarchy = NodeHierarchy::with_capacity(item_location_tree.children().len());

    item_location_tree
        .children()
        .iter()
        .for_each(|child_item_location_tree| {
            let child_item_location = child_item_location_tree.item_location();
            let child_node_id = node_id_from_item_location(child_item_location);
            node_id_to_item_locations.insert(child_node_id.clone(), child_item_location);

            let child_hierarchy = node_hierarchy_build_and_item_location_insert(
                child_item_location_tree,
                node_id_to_item_locations,
            );
            node_hierarchy.insert(child_node_id, child_hierarchy);
        });

    node_hierarchy
}
