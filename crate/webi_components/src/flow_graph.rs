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
        move |()| async move { flow_graph_src().await.unwrap() },
    );
    let dot_source_result = {
        move || {
            let dot_source = dot_source_resource
                .get()
                .unwrap_or_else(|| String::from("digraph { a -> b; }"));

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
    }
}

#[leptos::server(endpoint = "/flow_graph")]
pub async fn flow_graph_src() -> Result<String, ServerFnError<NoCustomError>> {
    Ok(String::from("digraph { a; b; c; a -> c; b -> c; }"))
}
