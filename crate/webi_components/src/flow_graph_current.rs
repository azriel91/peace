use dot_ix::{
    model::{common::GraphvizDotTheme, info_graph::InfoGraph},
    rt::IntoGraphvizDotSrc,
    web_components::DotSvg,
};
use leptos::{
    component, server, view, IntoView, ServerFnError, SignalGetUntracked, SignalSet, Transition,
};

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
    let (progress_info_graph_get, progress_info_graph_set) =
        leptos::create_signal(InfoGraph::default());
    let (progress_dot_src_and_styles, progress_dot_src_and_styles_set) =
        leptos::create_signal(None);

    let (outcome_info_graph_get, outcome_info_graph_set) =
        leptos::create_signal(InfoGraph::default());
    let (outcome_dot_src_and_styles, outcome_dot_src_and_styles_set) = leptos::create_signal(None);

    leptos::create_local_resource(
        move || (),
        move |()| async move {
            use gloo_timers::future::TimeoutFuture;

            loop {
                let (progress_info_graph, outcome_info_graph) =
                    info_graphs_fetch().await.unwrap_or_default();

                // Progress
                let progress_dot_src_and_styles =
                    IntoGraphvizDotSrc::into(&progress_info_graph, &GraphvizDotTheme::default());

                if progress_info_graph != progress_info_graph_get.get_untracked() {
                    progress_info_graph_set.set(progress_info_graph);
                    progress_dot_src_and_styles_set.set(Some(progress_dot_src_and_styles));
                }

                // Outcome
                let outcome_dot_src_and_styles =
                    IntoGraphvizDotSrc::into(&outcome_info_graph, &GraphvizDotTheme::default());

                if outcome_info_graph != outcome_info_graph_get.get_untracked() {
                    if let Ok(outcome_info_graph_serialized) =
                        serde_yaml::to_string(&outcome_info_graph)
                    {
                        leptos::logging::log!("{outcome_info_graph_serialized}");
                    }

                    outcome_info_graph_set.set(outcome_info_graph);
                    outcome_dot_src_and_styles_set.set(Some(outcome_dot_src_and_styles));
                }

                TimeoutFuture::new(250).await;
            }
        },
    );

    view! {
        <div class="flex items-center justify-center">
            <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
                <DotSvg
                    info_graph=progress_info_graph_get.into()
                    dot_src_and_styles=progress_dot_src_and_styles.into()
                />
                <DotSvg
                    info_graph=outcome_info_graph_get.into()
                    dot_src_and_styles=outcome_dot_src_and_styles.into()
                />
            </Transition>
        </div>
    }
}

#[server]
async fn info_graphs_fetch() -> Result<(InfoGraph, InfoGraph), ServerFnError> {
    use std::sync::{Arc, Mutex};

    use peace_cmd_model::CmdExecutionId;
    use peace_webi_model::{FlowOutcomeInfoGraphs, FlowProgressInfoGraphs};

    let cmd_execution_id = leptos::use_context::<Arc<Mutex<Option<CmdExecutionId>>>>();
    let flow_progress_info_graphs = leptos::use_context::<FlowProgressInfoGraphs<CmdExecutionId>>();
    let flow_outcome_info_graphs = leptos::use_context::<FlowOutcomeInfoGraphs<CmdExecutionId>>();

    if let Some(((cmd_execution_id, flow_progress_info_graphs), flow_outcome_info_graphs)) =
        cmd_execution_id
            .zip(flow_progress_info_graphs)
            .zip(flow_outcome_info_graphs)
    {
        let cmd_execution_id = cmd_execution_id.lock().ok().as_deref().copied().flatten();
        let flow_progress_info_graphs = flow_progress_info_graphs.lock().ok();

        let progress_info_graph = cmd_execution_id
            .zip(flow_progress_info_graphs)
            .and_then(|(cmd_execution_id, flow_progress_info_graphs)| {
                flow_progress_info_graphs.get(&cmd_execution_id).cloned()
            })
            .unwrap_or_else(InfoGraph::default);

        let flow_outcome_info_graphs = flow_outcome_info_graphs.lock().ok();
        let outcome_info_graph = cmd_execution_id
            .zip(flow_outcome_info_graphs)
            .and_then(|(cmd_execution_id, flow_outcome_info_graphs)| {
                flow_outcome_info_graphs.get(&cmd_execution_id).cloned()
            })
            .unwrap_or_else(InfoGraph::default);

        Ok((progress_info_graph, outcome_info_graph))
    } else {
        Ok((InfoGraph::default(), InfoGraph::default()))
    }
}
