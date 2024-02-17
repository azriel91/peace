use dot_ix::{model::common::DotSrcAndStyles, web_components::DotSvg};
use leptos::{
    component, create_signal, server_fn::error::NoCustomError, view, IntoView, ServerFnError,
    SignalGet, SignalUpdate, Transition,
};

/// Renders the flow graph.
#[component]
pub fn FlowGraph() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let dot_source_resource = leptos::create_resource(
        || (),
        move |()| async move { progress_dot_graph().await.unwrap() },
    );

    let progress_dot_graph = move || {
        let progress_dot_graph = dot_source_resource
            .get()
            .expect("Expected `progress_dot_graph` to always be generated successfully.");

        Some(progress_dot_graph)
    };

    view! {
        <div class="flex items-center justify-center">

            <div id="flow_dot_diagram"></div>
            <br />
            <div>
                "hello leptos!" { move || count.get() }
                <br />
                <button on:click=move |_| { set_count.update(|n| *n += 1); }>
                    "Increment me!"
                </button>
            </div>
        </div>
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <DotSvg dot_src_and_styles=progress_dot_graph />
        </Transition>
    }
}

#[leptos::server(endpoint = "/flow_graph")]
pub async fn progress_dot_graph() -> Result<DotSrcAndStyles, ServerFnError<NoCustomError>> {
    use dot_ix::{model::common::GraphvizDotTheme, rt::IntoGraphvizDotSrc};
    use peace_flow_model::FlowSpecInfo;

    let flow_spec_info = leptos::use_context::<FlowSpecInfo>().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("`FlowSpecInfo` was not set.".to_string())
    })?;

    let progress_info_graph = flow_spec_info.into_progress_info_graph();
    Ok(IntoGraphvizDotSrc::into(
        &progress_info_graph,
        &GraphvizDotTheme::default(),
    ))
}
