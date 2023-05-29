use dioxus::prelude::*;
use peace::rt_model::Flow;

use crate::{model::EnvManError, web::FlowDotRenderer};

/// Parameters for the flow graph.
#[derive(PartialEq, Props)]
pub struct FlowGraphProps {
    pub flow: Flow<EnvManError>,
}

/// Renders the flow graph.
pub fn FlowGraph(cx: Scope<FlowGraphProps>) -> Element {
    // use peace::rt_model::fn_graph::daggy::petgraph::dot::{Config, Dot};
    // let dot_source = Dot::with_config(cx.props.flow.graph().graph(),
    // &[Config::EdgeNoLabel]);
    let dot_source = {
        let flow_dot_renderer = FlowDotRenderer::new();
        flow_dot_renderer.dot(&cx.props.flow)
    };
    let mut num = use_state(cx, || 0);

    cx.render(rsx! {
        div {
            class: "flex items-center justify-center",
            div {
                id: "flow_dot_diagram",

                "hello axum! {num}"
                button { onclick: move |_| num += 1, "Increment" }
            }
        }
        script {
            r#type: "module",
            "\
            import {{ Graphviz }} from \"https://cdn.jsdelivr.net/npm/@hpcc-js/wasm/dist/graphviz.js\";\n\
            \n\
            const graphviz = await Graphviz.load();\n\
            const dot_source = `{dot_source}`;\n\
            document.getElementById(\"flow_dot_diagram\").innerHTML =\n\
            graphviz.layout(dot_source, \"svg\", \"dot\");\n\
            "
        }
    })
}
