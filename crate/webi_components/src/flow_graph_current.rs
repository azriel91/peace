use dot_ix::{
    model::{common::GraphvizDotTheme, info_graph::InfoGraph},
    rt::IntoGraphvizDotSrc,
    web_components::DotSvg,
};
use leptos::{component, view, IntoView, ReadSignal, Signal, SignalGet, Transition};
use peace_cmd_model::CmdExecutionId;
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
pub fn FlowGraphCurrent() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center">
            <ProgressGraph />
            <OutcomeGraph />
        </div>
    }
}

#[component]
fn ProgressGraph() -> impl IntoView {
    let cmd_execution_id = leptos::use_context::<ReadSignal<Option<CmdExecutionId>>>();

    let progress_and_dot_src_resource = leptos::create_resource(
        move || cmd_execution_id.as_ref().map(SignalGet::get).flatten(),
        move |cmd_execution_id| async move {
            let flow_progress_info_graphs =
                leptos::use_context::<FlowProgressInfoGraphs<CmdExecutionId>>();
            if let Some((cmd_execution_id, flow_progress_info_graphs)) =
                cmd_execution_id.zip(flow_progress_info_graphs)
            {
                let cmd_execution_id = &cmd_execution_id;
                let flow_progress_info_graphs = flow_progress_info_graphs.lock().ok();
                let flow_progress_info_graph =
                    flow_progress_info_graphs.and_then(|flow_progress_info_graphs| {
                        flow_progress_info_graphs.get(cmd_execution_id).cloned()
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
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <DotSvg
                info_graph=progress_info_graph
                dot_src_and_styles
            />
        </Transition>
    }
}

#[component]
fn OutcomeGraph() -> impl IntoView {
    let flow_id = leptos::use_context::<ReadSignal<FlowId>>();

    let outcome_info_graph_resource = leptos::create_resource(
        move || flow_id.as_ref().map(SignalGet::get),
        move |flow_id| async move {
            let flow_outcome_info_graphs = leptos::use_context::<FlowOutcomeInfoGraphs<FlowId>>();

            if let Some(flow_outcome_info_graphs) = flow_outcome_info_graphs {
                let flow_outcome_info_graphs = flow_outcome_info_graphs.lock().ok();

                flow_id
                    .as_ref()
                    .zip(flow_outcome_info_graphs)
                    .and_then(|(flow_id, flow_outcome_info_graphs)| {
                        flow_outcome_info_graphs.get(flow_id).cloned()
                    })
                    .unwrap_or_else(InfoGraph::default)
            } else {
                InfoGraph::default()
            }
        },
    );
    let outcome_info_graph = Signal::from(move || {
        if let Some(info_graph) = outcome_info_graph_resource.get() {
            let serialized = serde_yaml::to_string(&info_graph)
                .unwrap_or("Failed to serialize info_graph".to_string());
            leptos::logging::log!("{serialized}");
        }

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
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <DotSvg
                info_graph=outcome_info_graph
                dot_src_and_styles=dot_src_and_styles
            />
        </Transition>
    }
}
