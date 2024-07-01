use dot_ix::{
    model::{common::DotSrcAndStyles, info_graph::InfoGraph},
    web_components::{DotSvg, FlexDiag},
};
use leptos::{
    component, server_fn::error::NoCustomError, view, IntoView, ServerFnError, Signal, SignalGet,
    Transition,
};

/// Renders the flow graph.
#[component]
pub fn FlowGraph() -> impl IntoView {
    let progress_dot_resource = leptos::create_resource(
        || (),
        move |()| async move { progress_dot_graph().await.unwrap() },
    );
    let progress_dot_graph = move || {
        let progress_dot_graph = progress_dot_resource
            .get()
            .expect("Expected `progress_dot_graph` to always be generated successfully.");

        Some(progress_dot_graph)
    };

    let outcome_info_graph_resource = leptos::create_resource(
        || (),
        move |()| async move { outcome_info_graph().await.unwrap() },
    );
    let outcome_info_graph = move || {
        let outcome_info_graph =
            Signal::from(move || outcome_info_graph_resource.get().unwrap_or_default());

        view! {
            <FlexDiag info_graph=outcome_info_graph />
        }
    };

    view! {
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <div class="flex items-center justify-center">
                <DotSvg
                    dot_src_and_styles=progress_dot_graph.into()
                />
                {outcome_info_graph}
            </div>
        </Transition>
    }
}

/// Returns the graph representing item execution progress.
#[leptos::server(endpoint = "/flow_graph")]
pub async fn progress_dot_graph() -> Result<DotSrcAndStyles, ServerFnError<NoCustomError>> {
    use dot_ix::{model::common::GraphvizDotTheme, rt::IntoGraphvizDotSrc};
    use peace_flow_model::FlowSpecInfo;

    let flow_spec_info = leptos::use_context::<FlowSpecInfo>().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("`FlowSpecInfo` was not set.".to_string())
    })?;

    let progress_info_graph = flow_spec_info.to_progress_info_graph();
    Ok(IntoGraphvizDotSrc::into(
        &progress_info_graph,
        &GraphvizDotTheme::default(),
    ))
}

/// Returns the graph representing item outcomes.
#[leptos::server(endpoint = "/flow_graph")]
pub async fn outcome_info_graph() -> Result<InfoGraph, ServerFnError<NoCustomError>> {
    use peace_flow_model::FlowSpecInfo;

    let flow_spec_info = leptos::use_context::<FlowSpecInfo>().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("`FlowSpecInfo` was not set.".to_string())
    })?;

    let outcome_info_graph = flow_spec_info.to_outcome_info_graph();
    Ok(outcome_info_graph)
}
