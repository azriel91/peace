use dot_ix::{
    model::{
        common::{GraphvizDotTheme, NodeHierarchy, NodeId},
        info_graph::InfoGraph,
    },
    rt::IntoGraphvizDotSrc,
    web_components::DotSvg,
};
use leptos::{component, view, IntoView, ReadSignal, Signal, SignalGet, SignalWith, Transition};
use peace_item_model::{
    ItemLocation, ItemLocationTree, ItemLocationType, ItemLocationsAndInteractions,
};
use peace_params::ParamsSpecs;
use peace_resource_rt::{resources::ts::SetUp, Resources};
use peace_rt_model::Flow;

/// Renders the flow graph.
///
/// # Future
///
/// * Take in whether any execution is running. Use that info to style
///   nodes/edges.
/// * Take in values so they can be rendered, or `WriteSignal`s, to notify the
///   component that will render values about which node is selected.
#[component]
pub fn FlowGraph<E>(
    flow: ReadSignal<Flow<E>>,
    params_specs: ReadSignal<ParamsSpecs>,
    resources: ReadSignal<Resources<SetUp>>,
) -> impl IntoView
where
    E: 'static,
{
    view! {
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <div class="flex items-center justify-center">
                <ProgressGraph flow />
                <OutcomeGraph flow params_specs resources />
            </div>
        </Transition>
    }
}

#[component]
pub fn ProgressGraph<E>(flow: ReadSignal<Flow<E>>) -> impl IntoView
where
    E: 'static,
{
    let progress_info_graph = Signal::from(move || {
        let flow_spec_info = flow.get().flow_spec_info();
        flow_spec_info.to_progress_info_graph()
    });

    let dot_src_and_styles = Signal::from(move || {
        let dot_src_and_styles =
            IntoGraphvizDotSrc::into(&progress_info_graph.get(), &GraphvizDotTheme::default());
        Some(dot_src_and_styles)
    });

    view! {
        <DotSvg
            info_graph=progress_info_graph
            dot_src_and_styles=dot_src_and_styles
        />
    }
}

#[component]
pub fn OutcomeGraph<E>(
    flow: ReadSignal<Flow<E>>,
    params_specs: ReadSignal<ParamsSpecs>,
    resources: ReadSignal<Resources<SetUp>>,
) -> impl IntoView
where
    E: 'static,
{
    let outcome_info_graph = Signal::from(move || {
        let outcome_info_graph = {
            let flow = flow.get();
            let params_specs = params_specs.get();
            resources.with(|resources| {
                let item_locations_and_interactions =
                    flow.item_locations_and_interactions_example(&params_specs, resources);
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
            })
        };

        outcome_info_graph
    });

    let dot_src_and_styles = Signal::from(move || {
        let dot_src_and_styles =
            IntoGraphvizDotSrc::into(&outcome_info_graph.get(), &GraphvizDotTheme::default());
        Some(dot_src_and_styles)
    });

    view! {
        <DotSvg
            info_graph=outcome_info_graph
            dot_src_and_styles=dot_src_and_styles
        />
    }
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
