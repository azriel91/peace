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
    view! {
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <div class="flex items-center justify-center">
                <ProgressGraph />
                <OutcomeGraph />
            </div>
        </Transition>
    }
}

#[component]
pub fn ProgressGraph() -> impl IntoView {
    let flow_id = leptos::use_context::<ReadSignal<FlowId>>();

    let progress_and_dot_src_resource = leptos::create_resource(
        move || flow_id.as_ref().map(SignalGet::get),
        move |flow_id| async move {
            let flow_progress_info_graphs = leptos::use_context::<FlowProgressInfoGraphs<FlowId>>();
            if let Some((flow_id, flow_progress_info_graphs)) =
                flow_id.zip(flow_progress_info_graphs)
            {
                let flow_id = &flow_id;
                let flow_progress_info_graphs = flow_progress_info_graphs.lock().ok();
                let flow_progress_info_graph =
                    flow_progress_info_graphs.and_then(|flow_progress_info_graphs| {
                        flow_progress_info_graphs.get(flow_id).cloned()
                    });

                let dot_src_and_styles =
                    flow_progress_info_graph
                        .as_ref()
                        .map(|flow_progress_info_graph| {
                            IntoGraphvizDotSrc::into(
                                flow_progress_info_graph,
                                &GraphvizDotTheme::default(),
                            )
                        });

                flow_progress_info_graph.zip(dot_src_and_styles)
            } else {
                None
            }
        },
    );

    let progress_info_graph = leptos::create_memo(move |_| {
        progress_and_dot_src_resource
            .get()
            .flatten()
            .unzip()
            .0
            .unwrap_or_else(InfoGraph::default)
    })
    .into();

    let dot_src_and_styles =
        leptos::create_memo(move |_| progress_and_dot_src_resource.get().flatten().unzip().1)
            .into();

    view! {
        <DotSvg
            info_graph=progress_info_graph
            dot_src_and_styles
        />
    }
}

#[component]
pub fn OutcomeGraph() -> impl IntoView {
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
    let outcome_info_graph = Signal::from(move || {
        outcome_info_graph_resource
            .get()
            .unwrap_or_else(InfoGraph::default)
    });

    let dot_src_and_styles = leptos::create_memo(move |_| {
        let dot_src_and_styles =
            IntoGraphvizDotSrc::into(&outcome_info_graph.get(), &GraphvizDotTheme::default());
        Some(dot_src_and_styles)
    })
    .into();

    view! {
        <DotSvg
            info_graph=outcome_info_graph
            dot_src_and_styles=dot_src_and_styles
        />
    }
}
