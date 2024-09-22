use dot_ix::{
    model::{common::GraphvizDotTheme, info_graph::InfoGraph},
    rt::IntoGraphvizDotSrc,
    web_components::DotSvg,
};
use leptos::{component, view, IntoView, ReadSignal, Signal, SignalGet, Transition};
use peace_core::FlowId;
use peace_webi_model::{FlowOutcomeInfoGraphs, FlowProgressInfoGraphs};

/// Renders the flow graph.
///
/// # Future
///
/// * Take in whether any execution is running. Use that info to style
///   nodes/edges.
/// * Take in values so they can be rendered, or `WriteSignal`s, to notify the
///   component that will render values about which node is selected.
#[component]
pub fn FlowGraph() -> impl IntoView {
    // TODO: when multiple flows are supported, set flow.
    let flow_id = leptos::use_context::<ReadSignal<FlowId>>();

    let outcome_info_graph_resource = leptos::create_resource(
        move || flow_id.as_ref().map(SignalGet::get),
        move |flow_id| async move {
            let flow_outcome_info_graphs =
                leptos::expect_context::<FlowOutcomeInfoGraphs<FlowId>>();
            let flow_outcome_info_graphs = flow_outcome_info_graphs.lock().ok();

            flow_id
                .as_ref()
                .zip(flow_outcome_info_graphs)
                .and_then(|(flow_id, flow_outcome_info_graphs)| {
                    flow_outcome_info_graphs.get(flow_id).cloned()
                })
                .unwrap_or_else(InfoGraph::default)
        },
    );
    let outcome_info_graph_example = Signal::from(move || {
        outcome_info_graph_resource
            .get()
            .unwrap_or_else(InfoGraph::default)
    });

    let progress_graph_maybe = move || match flow_id {
        Some(flow_id) => view! { <ProgressGraph flow_id /> }.into_view(),
        None => view! { <p>"No flow selected."</p> }.into_view(),
    };

    view! {
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <div class="flex items-center justify-center">
                {progress_graph_maybe}
                <OutcomeGraph outcome_info_graph=outcome_info_graph_example />
            </div>
        </Transition>
    }
}

#[component]
pub fn ProgressGraph(flow_id: ReadSignal<FlowId>) -> impl IntoView {
    let progress_info_graph_and_dot_src_and_styles = Signal::from(move || {
        let flow_progress_info_graphs = leptos::expect_context::<FlowProgressInfoGraphs<FlowId>>();

        let flow_id = flow_id.get();
        let flow_id = &flow_id;
        let flow_progress_info_graph = flow_progress_info_graphs
            .lock()
            .ok()
            .and_then(|flow_progress_info_graphs| flow_progress_info_graphs.get(flow_id).cloned());

        let dot_src_and_styles =
            flow_progress_info_graph
                .as_ref()
                .map(|flow_progress_info_graph| {
                    IntoGraphvizDotSrc::into(flow_progress_info_graph, &GraphvizDotTheme::default())
                });

        flow_progress_info_graph.zip(dot_src_and_styles)
    });

    match progress_info_graph_and_dot_src_and_styles.get() {
        Some((progress_info_graph, dot_src_and_styles)) => view! {
            <DotSvg
                info_graph=Signal::from(move || progress_info_graph.clone())
                dot_src_and_styles=Signal::from(move || Some(dot_src_and_styles.clone()))
            />
        }
        .into_view(),
        None => view! {
            "progress_info_graph or dot_src_and_styles is None."
        }
        .into_view(),
    }
}

#[component]
pub fn OutcomeGraph(outcome_info_graph: Signal<InfoGraph>) -> impl IntoView {
    let dot_src_and_styles = Signal::from(move || {
        let dot_src_and_styles =
            IntoGraphvizDotSrc::into(&outcome_info_graph.get(), &GraphvizDotTheme::default());
        Some(dot_src_and_styles)
    });

    view! {
        <DotSvg
            info_graph=outcome_info_graph.into()
            dot_src_and_styles=dot_src_and_styles
        />
    }
}
