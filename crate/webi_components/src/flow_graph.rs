use dot_ix::{model::common::DotSrcAndStyles, web_components::DotSvg};
use leptos::{
    component, server_fn::error::NoCustomError, view, IntoView, ServerFnError, SignalGet,
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

    let outcome_dot_resource = leptos::create_resource(
        || (),
        move |()| async move { outcome_dot_graph().await.unwrap() },
    );
    let outcome_dot_graph = move || {
        let outcome_dot_graph = outcome_dot_resource
            .get()
            .expect("Expected `outcome_dot_graph` to always be generated successfully.");

        Some(outcome_dot_graph)
    };

    view! {
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <div class="flex items-center justify-center">
                <DotSvg dot_src_and_styles=progress_dot_graph />
                <DotSvg dot_src_and_styles=outcome_dot_graph />
            </div>
        </Transition>
    }
}

/// Returns the graph representing item execution progress.
#[leptos::server(endpoint = "/flow_graph")]
pub async fn progress_dot_graph() -> Result<DotSrcAndStyles, ServerFnError<NoCustomError>> {
    use dot_ix::{
        model::common::{graphviz_dot_theme::GraphStyle, GraphvizDotTheme},
        rt::IntoGraphvizDotSrc,
    };
    use peace_flow_model::FlowSpecInfo;

    let flow_spec_info = leptos::use_context::<FlowSpecInfo>().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("`FlowSpecInfo` was not set.".to_string())
    })?;

    let progress_info_graph = flow_spec_info.to_progress_info_graph();
    Ok(IntoGraphvizDotSrc::into(
        &progress_info_graph,
        &GraphvizDotTheme::default().with_graph_style(GraphStyle::Circle),
    ))
}

/// Returns the graph representing item outcomes.
#[leptos::server(endpoint = "/flow_graph")]
pub async fn outcome_dot_graph() -> Result<DotSrcAndStyles, ServerFnError<NoCustomError>> {
    use dot_ix::{
        model::common::{graphviz_dot_theme::GraphStyle, GraphvizDotTheme},
        rt::IntoGraphvizDotSrc,
    };
    use peace_flow_model::FlowSpecInfo;

    let flow_spec_info = leptos::use_context::<FlowSpecInfo>().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("`FlowSpecInfo` was not set.".to_string())
    })?;

    let outcome_info_graph = flow_spec_info.to_outcome_info_graph();
    Ok(IntoGraphvizDotSrc::into(
        &outcome_info_graph,
        &GraphvizDotTheme::default().with_graph_style(GraphStyle::Boxes),
    ))
}
