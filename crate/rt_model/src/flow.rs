use peace_cfg::FlowId;
use peace_data::fn_graph::GraphInfo;
use peace_flow_model::{FlowSpecInfo, ItemSpecInfo};

use crate::ItemGraph;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "item_interactions", feature = "item_state_example"))] {
        use std::collections::{BTreeMap, BTreeSet};

        use indexmap::IndexMap;
        use peace_cfg::ItemId;
        use peace_item_model::{
            ItemInteraction,
            ItemInteractionsCurrentOrExample,
            ItemLocation,
            ItemLocationsAndInteractions,
            ItemLocationTree,
        };
        use peace_params::ParamsSpecs;
        use peace_resource_rt::{resources::ts::SetUp, Resources};
    }
}

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

    /// Generates a `FlowSpecInfo` from this `Flow`'s information.
    pub fn flow_spec_info(&self) -> FlowSpecInfo
    where
        E: 'static,
    {
        let flow_id = self.flow_id.clone();
        let graph_info = GraphInfo::from_graph(&self.graph, |item_boxed| {
            let item_id = item_boxed.id().clone();
            ItemSpecInfo { item_id }
        });

        FlowSpecInfo::new(flow_id, graph_info)
    }

    #[cfg(all(feature = "item_interactions", feature = "item_state_example"))]
    pub fn item_locations_and_interactions_example(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> ItemLocationsAndInteractions
    where
        E: 'static,
    {
        // Build the flattened hierarchy.
        //
        // Regardless of how nested each `ItemLocation` is, the map will have an entry
        // for it.
        //
        // The entry key is the `ItemLocation`, and the values are a list of its direct
        // descendent `ItemLocation`s.
        //
        // This means a lot of cloning of `ItemLocation`s.

        let (item_location_direct_descendents, item_to_item_interactions) = self
            .graph()
            .iter()
            // Note: This will silently drop the item locations if `interactions_example` fails to
            // return.
            .filter_map(|item| {
                item.interactions_example(params_specs, resources)
                    .ok()
                    .map(|item_interactions_example| (item.id().clone(), item_interactions_example))
            })
            .fold(
                (
                    BTreeMap::<ItemLocation, BTreeSet<ItemLocation>>::new(),
                    IndexMap::<ItemId, Vec<ItemInteraction>>::with_capacity(
                        self.graph().node_count(),
                    ),
                ),
                |(mut item_location_direct_descendents, mut item_to_item_interactions),
                 (item_id, item_interactions_example)| {
                    item_interactions_example
                        .iter()
                        .for_each(|item_interaction| match &item_interaction {
                            ItemInteraction::Push(item_interaction_push) => {
                                item_location_descendents_insert(
                                    &mut item_location_direct_descendents,
                                    item_interaction_push.location_from(),
                                );
                                item_location_descendents_insert(
                                    &mut item_location_direct_descendents,
                                    item_interaction_push.location_to(),
                                );
                            }
                            ItemInteraction::Pull(item_interaction_pull) => {
                                item_location_descendents_insert(
                                    &mut item_location_direct_descendents,
                                    item_interaction_pull.location_client(),
                                );
                                item_location_descendents_insert(
                                    &mut item_location_direct_descendents,
                                    item_interaction_pull.location_server(),
                                );
                            }
                            ItemInteraction::Within(item_interaction_within) => {
                                item_location_descendents_insert(
                                    &mut item_location_direct_descendents,
                                    item_interaction_within.location(),
                                );
                            }
                        });

                    item_to_item_interactions
                        .insert(item_id, item_interactions_example.into_inner());

                    (item_location_direct_descendents, item_to_item_interactions)
                },
            );

        let item_locations_top_level = item_location_direct_descendents
            .keys()
            .filter(|item_location| {
                // this item_location is not in any descendents
                !item_location_direct_descendents
                    .values()
                    .any(|item_location_descendents| {
                        item_location_descendents.contains(item_location)
                    })
            })
            .cloned()
            .collect::<Vec<ItemLocation>>();

        let item_locations_top_level_len = item_locations_top_level.len();
        let (_item_location_direct_descendents, item_location_trees) =
            item_locations_top_level.into_iter().fold(
                (
                    item_location_direct_descendents,
                    Vec::with_capacity(item_locations_top_level_len),
                ),
                |(mut item_location_direct_descendents, mut item_location_trees), item_location| {
                    let item_location_tree = item_location_tree_collect(
                        &mut item_location_direct_descendents,
                        item_location,
                    );
                    item_location_trees.push(item_location_tree);

                    (item_location_direct_descendents, item_location_trees)
                },
            );

        let item_location_count = item_location_trees.iter().fold(
            item_location_trees.len(),
            |item_location_count_acc, item_location_tree| {
                item_location_count_acc + item_location_tree.item_location_count()
            },
        );

        ItemLocationsAndInteractions::new(
            item_location_trees,
            item_to_item_interactions,
            item_location_count,
        )
    }

    #[cfg(all(feature = "item_interactions", feature = "item_state_example"))]
    pub fn item_locations_and_interactions_current(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> ItemLocationsAndInteractions
    where
        E: 'static,
    {
        // Build the flattened hierarchy.
        //
        // Regardless of how nested each `ItemLocation` is, the map will have an entry
        // for it.
        //
        // The entry key is the `ItemLocation`, and the values are a list of its direct
        // descendent `ItemLocation`s.
        //
        // This means a lot of cloning of `ItemLocation`s.

        let (item_location_direct_descendents, item_to_item_interactions) = self
            .graph()
            .iter()
            // Note: This will silently drop the item locations if `interactions_try_current` fails
            // to return.
            .filter_map(|item| {
                item.interactions_try_current(params_specs, resources)
                    .ok()
                    .map(|item_interactions_current_or_example| {
                        (item.id().clone(), item_interactions_current_or_example)
                    })
            })
            .fold(
                (
                    BTreeMap::<ItemLocation, BTreeSet<ItemLocation>>::new(),
                    IndexMap::<ItemId, Vec<ItemInteraction>>::with_capacity(
                        self.graph().node_count(),
                    ),
                ),
                |(mut item_location_direct_descendents, mut item_to_item_interactions),
                 (item_id, item_interactions_current_or_example)| {
                    // TODO: we need to hide the nodes if they came from `Example`.
                    let item_interactions_current_or_example =
                        match item_interactions_current_or_example {
                            ItemInteractionsCurrentOrExample::Current(
                                item_interactions_current,
                            ) => item_interactions_current.into_inner(),
                            ItemInteractionsCurrentOrExample::Example(
                                item_interactions_example,
                            ) => item_interactions_example.into_inner(),
                        };

                    item_interactions_current_or_example
                        .iter()
                        .for_each(|item_interaction| match &item_interaction {
                            ItemInteraction::Push(item_interaction_push) => {
                                item_location_descendents_insert(
                                    &mut item_location_direct_descendents,
                                    item_interaction_push.location_from(),
                                );
                                item_location_descendents_insert(
                                    &mut item_location_direct_descendents,
                                    item_interaction_push.location_to(),
                                );
                            }
                            ItemInteraction::Pull(item_interaction_pull) => {
                                item_location_descendents_insert(
                                    &mut item_location_direct_descendents,
                                    item_interaction_pull.location_client(),
                                );
                                item_location_descendents_insert(
                                    &mut item_location_direct_descendents,
                                    item_interaction_pull.location_server(),
                                );
                            }
                            ItemInteraction::Within(item_interaction_within) => {
                                item_location_descendents_insert(
                                    &mut item_location_direct_descendents,
                                    item_interaction_within.location(),
                                );
                            }
                        });

                    item_to_item_interactions.insert(item_id, item_interactions_current_or_example);

                    (item_location_direct_descendents, item_to_item_interactions)
                },
            );

        let item_locations_top_level = item_location_direct_descendents
            .keys()
            .filter(|item_location| {
                // this item_location is not in any descendents
                !item_location_direct_descendents
                    .values()
                    .any(|item_location_descendents| {
                        item_location_descendents.contains(item_location)
                    })
            })
            .cloned()
            .collect::<Vec<ItemLocation>>();

        let item_locations_top_level_len = item_locations_top_level.len();
        let (_item_location_direct_descendents, item_location_trees) =
            item_locations_top_level.into_iter().fold(
                (
                    item_location_direct_descendents,
                    Vec::with_capacity(item_locations_top_level_len),
                ),
                |(mut item_location_direct_descendents, mut item_location_trees), item_location| {
                    let item_location_tree = item_location_tree_collect(
                        &mut item_location_direct_descendents,
                        item_location,
                    );
                    item_location_trees.push(item_location_tree);

                    (item_location_direct_descendents, item_location_trees)
                },
            );

        let item_location_count = item_location_trees.iter().fold(
            item_location_trees.len(),
            |item_location_count_acc, item_location_tree| {
                item_location_count_acc + item_location_tree.item_location_count()
            },
        );

        ItemLocationsAndInteractions::new(
            item_location_trees,
            item_to_item_interactions,
            item_location_count,
        )
    }
}

/// Recursively constructs an `ItemLocationTree`.
#[cfg(all(feature = "item_interactions", feature = "item_state_example"))]
fn item_location_tree_collect(
    item_location_direct_descendents: &mut BTreeMap<ItemLocation, BTreeSet<ItemLocation>>,
    item_location_parent: ItemLocation,
) -> ItemLocationTree {
    match item_location_direct_descendents.remove_entry(&item_location_parent) {
        Some((item_location, item_location_children)) => {
            let children = item_location_children
                .into_iter()
                .map(|item_location_child| {
                    item_location_tree_collect(
                        item_location_direct_descendents,
                        item_location_child,
                    )
                })
                .collect::<Vec<ItemLocationTree>>();
            ItemLocationTree::new(item_location, children)
        }

        // Should never be reached.
        None => ItemLocationTree::new(item_location_parent, Vec::new()),
    }
}

/// Inserts / extends the `item_location_direct_descendents` with an entry for
/// each `ItemLocation` and its direct `ItemLocation` descendents.
#[cfg(all(feature = "item_interactions", feature = "item_state_example"))]
fn item_location_descendents_insert(
    item_location_direct_descendents: &mut BTreeMap<ItemLocation, BTreeSet<ItemLocation>>,
    item_location_ancestors: &[ItemLocation],
) {
    // Each subsequent `ItemLocation` in `location_from` is a child of the previous
    // `ItemLocation`.
    let item_location_iter = item_location_ancestors.iter();
    let item_location_child_iter = item_location_ancestors.iter().skip(1);

    item_location_iter.zip(item_location_child_iter).for_each(
        |(item_location, item_location_child)| {
            // Save one clone by not using `BTreeSet::entry`
            if let Some(item_location_children) =
                item_location_direct_descendents.get_mut(item_location)
            {
                if !item_location_children.contains(item_location_child) {
                    item_location_children.insert(item_location_child.clone());
                }
            } else {
                let mut item_location_children = BTreeSet::new();
                item_location_children.insert(item_location_child.clone());
                item_location_direct_descendents
                    .insert(item_location.clone(), item_location_children);
            }

            // Add an empty set for the child `ItemLocation`.
            if !item_location_direct_descendents.contains_key(item_location_child) {
                item_location_direct_descendents
                    .insert(item_location_child.clone(), BTreeSet::new());
            }
        },
    );
}
