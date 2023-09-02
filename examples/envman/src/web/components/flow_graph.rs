use leptos::{
    component, create_signal, view, IntoView, ServerFnError, SignalGet, SignalUpdate, Transition,
};

/// Renders the flow graph.
#[component]
pub fn FlowGraph() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let dot_source_resource = leptos::create_resource(
        || (),
        move |()| async move { flow_graph_src().await.unwrap() },
    );
    let dot_source_result = {
        move || {
            let dot_source = dot_source_resource
                .get()
                .unwrap_or_else(|| String::from("digraph {}"));

            let script_src = format!(
                "\
                import {{ Graphviz }} from \"https://cdn.jsdelivr.net/npm/@hpcc-js/wasm/dist/graphviz.js\";\n\
                \n\
                const graphviz = await Graphviz.load();\n\
                const dot_source = `{dot_source}`;\n\
                document.getElementById(\"flow_dot_diagram\").innerHTML =\n\
                graphviz.layout(dot_source, \"svg\", \"dot\");\n\
                "
            );

            view! {
                <script type="module">
                { script_src }
                </script>
            }
        }
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
        { dot_source_result }
        </Transition>
        // Client side rendering, if we know what flow we have.
        //
        // This doesn't work for us because:
        //
        // * Loading a `flow` is `async`.
        // * Component functions are `sync`.
        // * Loading a resource using `leptos::create_resource` returns the loaded value between
        //   `<!-- leptos -->` tags.
        // * Such tags cannot be children of a `<script />` element.
        //
        // ```html
        // <script type="module">
        //     "\
        //     import {{ Graphviz }} from \"https://cdn.jsdelivr.net/npm/@hpcc-js/wasm/dist/graphviz.js\";\n\
        //     \n\
        //     const graphviz = await Graphviz.load();\n\
        //     const dot_source = `" { dot_source } "`;\n\
        //     document.getElementById(\"flow_dot_diagram\").innerHTML =\n\
        //     graphviz.layout(dot_source, \"svg\", \"dot\");\n\
        //     "
        // </script>
        // ```
    }
}

#[leptos::server(FlowGraphSrc, "/flow_graph")]
pub async fn flow_graph_src() -> Result<String, ServerFnError> {
    use crate::{flows::AppUploadFlow, web::FlowDotRenderer};
    let flow = AppUploadFlow::flow()
        .await
        .map_err(|envman_error| ServerFnError::ServerError(format!("{}", envman_error)))?;

    // use peace::rt_model::fn_graph::daggy::petgraph::dot::{Config, Dot};
    // let dot_source = Dot::with_config(cx.props.flow.graph().graph(),
    // &[Config::EdgeNoLabel]);

    let flow_dot_renderer = FlowDotRenderer::new();
    Ok(flow_dot_renderer.dot(&flow))
}
