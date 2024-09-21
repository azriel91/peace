use dot_ix_model::{
    common::{NodeHierarchy, NodeId},
    info_graph::InfoGraph,
};
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
    } = item_locations_and_interactions;

    let node_hierarchy = item_location_trees
        .iter()
        .map(|item_location_tree| {
            let item_location = item_location_tree.item_location();
            let node_id = node_id_from_item_location(item_location);
            (
                node_id,
                node_hierarchy_from_item_location_tree(item_location_tree),
            )
        })
        .fold(
            NodeHierarchy::with_capacity(item_location_trees.len()),
            |mut node_hierarchy_all, (node_id, node_hierarchy_top_level)| {
                node_hierarchy_all.insert(node_id, node_hierarchy_top_level);
                node_hierarchy_all
            },
        );

    InfoGraph::default().with_hierarchy(node_hierarchy)
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

fn node_hierarchy_from_item_location_tree(item_location_tree: &ItemLocationTree) -> NodeHierarchy {
    let mut node_hierarchy = NodeHierarchy::with_capacity(item_location_tree.children().len());

    item_location_tree
        .children()
        .iter()
        .for_each(|item_location_tree_child| {
            let child_node_id =
                node_id_from_item_location(item_location_tree_child.item_location());
            let child_hierarchy = node_hierarchy_from_item_location_tree(item_location_tree_child);
            node_hierarchy.insert(child_node_id, child_hierarchy);
        });

    node_hierarchy
}
