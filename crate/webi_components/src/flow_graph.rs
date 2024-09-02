use dot_ix::{
    model::{common::DotSrcAndStyles, info_graph::InfoGraph},
    web_components::DotSvg,
};
use leptos::{
    component, server_fn::error::NoCustomError, view, IntoView, ServerFnError, Signal, SignalGet,
    Transition,
};

/// Renders the flow graph.
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
    let progress_dot_resource = leptos::create_resource(
        || (),
        move |()| async move { progress_dot_graph().await.unwrap() },
    );

    let progress_info_graph = move || {
        let progress_info_graph_and_dot_src_and_styles = progress_dot_resource
            .get()
            .expect("Expected `progress_info_graph_and_dot_src_and_styles` to always be generated successfully.");

        progress_info_graph_and_dot_src_and_styles.0
    };

    let progress_dot_graph = move || {
        let progress_info_graph_and_dot_src_and_styles = progress_dot_resource
            .get()
            .expect("Expected `progress_info_graph_and_dot_src_and_styles` to always be generated successfully.");

        Some(progress_info_graph_and_dot_src_and_styles.1)
    };

    view! {
        <DotSvg
            info_graph=progress_info_graph.into()
            dot_src_and_styles=progress_dot_graph.into()
        />
    }
}

#[component]
pub fn OutcomeGraph() -> impl IntoView {
    let outcome_info_graph_resource = leptos::create_resource(
        || (),
        move |()| async move { outcome_info_graph().await.unwrap() },
    );

    let outcome_info_graph = Signal::from(move || {
        outcome_info_graph_resource
            .get()
            .map(|outcome_info_graph_and_dot_src_and_styles| {
                outcome_info_graph_and_dot_src_and_styles.0
            })
            .unwrap_or_default()
    });

    let outcome_dot_graph = Signal::from(move || {
        outcome_info_graph_resource
            .get()
            .map(|outcome_info_graph_and_dot_src_and_styles| {
                outcome_info_graph_and_dot_src_and_styles.1
            })
    });

    view! {
        <DotSvg
            info_graph=outcome_info_graph.into()
            dot_src_and_styles=outcome_dot_graph.into()
        />
    }
}

/// Returns the graph representing item execution progress.
#[leptos::server(endpoint = "/flow_graph")]
pub async fn progress_dot_graph()
-> Result<(InfoGraph, DotSrcAndStyles), ServerFnError<NoCustomError>> {
    use dot_ix::{model::common::GraphvizDotTheme, rt::IntoGraphvizDotSrc};
    use peace_flow_model::FlowSpecInfo;

    let flow_spec_info = leptos::use_context::<FlowSpecInfo>().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("`FlowSpecInfo` was not set.".to_string())
    })?;

    let progress_info_graph = flow_spec_info.to_progress_info_graph();
    let dot_src_and_styles =
        IntoGraphvizDotSrc::into(&progress_info_graph, &GraphvizDotTheme::default());
    Ok((progress_info_graph, dot_src_and_styles))
}

/// Returns the graph representing item outcomes.
#[leptos::server(endpoint = "/flow_graph")]
pub async fn outcome_info_graph()
-> Result<(InfoGraph, DotSrcAndStyles), ServerFnError<NoCustomError>> {
    use dot_ix::{model::common::GraphvizDotTheme, rt::IntoGraphvizDotSrc};
    use peace_flow_model::FlowSpecInfo;

    let flow_spec_info = leptos::use_context::<FlowSpecInfo>().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("`FlowSpecInfo` was not set.".to_string())
    })?;

    let outcome_info_graph = flow_spec_info.to_outcome_info_graph();
    let dot_src_and_styles =
        IntoGraphvizDotSrc::into(&outcome_info_graph, &GraphvizDotTheme::default());
    Ok((outcome_info_graph, dot_src_and_styles))
}
