use dot_ix::{
    model::{common::GraphvizDotTheme, info_graph::InfoGraph},
    rt::IntoGraphvizDotSrc,
    web_components::DotSvg,
};
use leptos::{component, server, view, IntoView, ServerFnError, SignalSet, Transition};

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

#[server]
async fn progress_info_graph_fetch() -> Result<InfoGraph, ServerFnError> {
    use std::sync::{Arc, Mutex};

    use peace_cmd_model::CmdExecutionId;
    use peace_webi_model::FlowProgressInfoGraphs;

    let cmd_execution_id = leptos::use_context::<Arc<Mutex<Option<CmdExecutionId>>>>();
    let flow_progress_info_graphs = leptos::use_context::<FlowProgressInfoGraphs<CmdExecutionId>>();
    let progress_info_graph = if let Some((cmd_execution_id, flow_progress_info_graphs)) =
        cmd_execution_id.zip(flow_progress_info_graphs)
    {
        let cmd_execution_id = cmd_execution_id.lock().ok().as_deref().copied().flatten();
        let flow_progress_info_graphs = flow_progress_info_graphs.lock().ok();

        cmd_execution_id
            .zip(flow_progress_info_graphs)
            .and_then(|(cmd_execution_id, flow_progress_info_graphs)| {
                flow_progress_info_graphs.get(&cmd_execution_id).cloned()
            })
            .unwrap_or_else(InfoGraph::default)
    } else {
        InfoGraph::default()
    };

    Ok(progress_info_graph)
}

#[component]
fn ProgressGraph() -> impl IntoView {
    let (progress_info_graph, progress_info_graph_set) =
        leptos::create_signal(InfoGraph::default());
    let (dot_src_and_styles, dot_src_and_styles_set) = leptos::create_signal(None);

    leptos::create_local_resource(
        move || (),
        move |()| async move {
            loop {
                use gloo_timers::future::TimeoutFuture;

                let progress_info_graph = progress_info_graph_fetch().await.unwrap_or_default();
                let dot_src_and_styles =
                    IntoGraphvizDotSrc::into(&progress_info_graph, &GraphvizDotTheme::default());

                progress_info_graph_set.set(progress_info_graph);
                dot_src_and_styles_set.set(Some(dot_src_and_styles));

                TimeoutFuture::new(500).await;
            }
        },
    );

    view! {
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <DotSvg
                info_graph=progress_info_graph.into()
                dot_src_and_styles=dot_src_and_styles.into()
            />
        </Transition>
    }
}

#[server]
async fn outcome_info_graph_fetch() -> Result<InfoGraph, ServerFnError> {
    use std::sync::{Arc, Mutex};

    use peace_cmd_model::CmdExecutionId;
    use peace_webi_model::FlowOutcomeInfoGraphs;

    let cmd_execution_id = leptos::use_context::<Arc<Mutex<Option<CmdExecutionId>>>>();
    let flow_outcome_info_graphs = leptos::use_context::<FlowOutcomeInfoGraphs<CmdExecutionId>>();
    let outcome_info_graph = if let Some((cmd_execution_id, flow_outcome_info_graphs)) =
        cmd_execution_id.zip(flow_outcome_info_graphs)
    {
        let cmd_execution_id = cmd_execution_id.lock().ok().as_deref().copied().flatten();
        let flow_outcome_info_graphs = flow_outcome_info_graphs.lock().ok();

        match cmd_execution_id {
            Some(cmd_execution_id) => leptos::logging::log!(
                "fetching outcome_info_graph for cmd_execution_id: {cmd_execution_id:?}"
            ),
            None => leptos::logging::log!("No cmd_execution_id in leptos context."),
        }

        cmd_execution_id
            .zip(flow_outcome_info_graphs)
            .and_then(|(cmd_execution_id, flow_outcome_info_graphs)| {
                leptos::logging::log!(
                    "rendering outcome graph for cmd_execution_id: {cmd_execution_id:?}"
                );

                flow_outcome_info_graphs.get(&cmd_execution_id).cloned()
            })
            .unwrap_or_else(InfoGraph::default)
    } else {
        InfoGraph::default()
    };

    Ok(outcome_info_graph)
}

#[component]
fn OutcomeGraph() -> impl IntoView {
    let (outcome_info_graph, outcome_info_graph_set) = leptos::create_signal(InfoGraph::default());
    let (dot_src_and_styles, dot_src_and_styles_set) = leptos::create_signal(None);

    leptos::create_local_resource(
        move || (),
        move |()| async move {
            use gloo_timers::future::TimeoutFuture;

            leptos::logging::log!("on_load for OutcomeGraph");

            loop {
                let outcome_info_graph = outcome_info_graph_fetch().await.unwrap_or_default();
                let dot_src_and_styles =
                    IntoGraphvizDotSrc::into(&outcome_info_graph, &GraphvizDotTheme::default());

                outcome_info_graph_set.set(outcome_info_graph);
                dot_src_and_styles_set.set(Some(dot_src_and_styles));

                TimeoutFuture::new(500).await;
            }
        },
    );

    view! {
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <DotSvg
                info_graph=outcome_info_graph.into()
                dot_src_and_styles=dot_src_and_styles.into()
            />
        </Transition>
    }
}
