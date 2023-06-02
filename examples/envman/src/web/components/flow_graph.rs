use leptos::{component, create_signal, view, IntoView, Scope, SignalGet, SignalUpdate};
use peace::rt_model::Flow;

use crate::{model::EnvManError, web::FlowDotRenderer};

/// Renders the flow graph.
#[component]
pub fn FlowGraph(cx: Scope, flow: Flow<EnvManError>) -> impl IntoView {
    // use peace::rt_model::fn_graph::daggy::petgraph::dot::{Config, Dot};
    // let dot_source = Dot::with_config(cx.props.flow.graph().graph(),
    // &[Config::EdgeNoLabel]);
    let dot_source = {
        let flow_dot_renderer = FlowDotRenderer::new();
        flow_dot_renderer.dot(&flow)
    };
    let (count, set_count) = create_signal(cx, 0);

    view! {
        cx,
        <div class="flex items-center justify-center">
            <div id="flow_dot_diagram"></div>
            <div>
                "hello leptos!" { move || count.get() }
                <button on:click=move |_| { set_count.update(|n| *n += 1); }>
                    "Increment me!"
                </button>
            </div>
        </div>
        <script type="module">
            "\
            import { Graphviz } from \"https://cdn.jsdelivr.net/npm/@hpcc-js/wasm/dist/graphviz.js\";\n\
            \n\
            const graphviz = await Graphviz.load();\n\
            const dot_source = `" { dot_source }"`;\n\
            document.getElementById(\"flow_dot_diagram\").innerHTML =\n\
            graphviz.layout(dot_source, \"svg\", \"dot\");\n\
            "
        </script>
    }
}
