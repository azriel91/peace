use dot_ix::{
    model::{common::GraphvizDotTheme, info_graph::InfoGraph},
    rt::IntoGraphvizDotSrc,
    web_components::DotSvg,
};
use leptos::{component, view, IntoView, ReadSignal, Signal, SignalGet, Transition};
use peace_core::FlowId;
use peace_rt_model::Flow;
use peace_webi_model::FlowInfoGraphs;

/// Renders the flow graph.
///
/// # Future
///
/// * Take in whether any execution is running. Use that info to style
///   nodes/edges.
/// * Take in values so they can be rendered, or `WriteSignal`s, to notify the
///   component that will render values about which node is selected.
#[component]
pub fn FlowGraph<E>(flow: ReadSignal<Option<Flow<E>>>) -> impl IntoView
where
    E: 'static,
{
    // TODO: when multiple flows are supported, set flow.
    let outcome_info_graph_resource = leptos::create_resource(
        move || flow.get(),
        move |flow| async move {
            let flow_info_graphs = leptos::expect_context::<FlowInfoGraphs<FlowId>>();
            let flow_id = flow.as_ref().map(Flow::flow_id);
            let flow_info_graphs = flow_info_graphs.lock().ok();

            flow_id
                .zip(flow_info_graphs)
                .and_then(|(flow_id, flow_info_graphs)| flow_info_graphs.get(flow_id).cloned())
                .unwrap_or_else(InfoGraph::default)
        },
    );
    let outcome_info_graph_example = Signal::from(move || {
        outcome_info_graph_resource
            .get()
            .unwrap_or_else(InfoGraph::default)
    });

    let progress_graph_maybe = move || {
        if flow.get().is_some() {
            view! {
                <ProgressGraph flow />
            }
            .into_view()
        } else {
            view! {
                <p>"No flow selected."</p>
            }
            .into_view()
        }
    };

    view! {
        <Transition fallback=move || view! { <p>"Loading graph..."</p> }>
            <div class="flex items-center justify-center">
                {progress_graph_maybe}
                <OutcomeGraph outcome_info_graph=outcome_info_graph_example />
            </div>
        </Transition>
    }
}

#[component]
pub fn ProgressGraph<E>(flow: ReadSignal<Option<Flow<E>>>) -> impl IntoView
where
    E: 'static,
{
    let progress_info_graph = Signal::from(move || {
        let flow_spec_info = flow
            .get()
            .expect("Expected flow to be set.")
            .flow_spec_info();
        flow_spec_info.to_progress_info_graph()
    });

    let dot_src_and_styles = Signal::from(move || {
        let dot_src_and_styles =
            IntoGraphvizDotSrc::into(&progress_info_graph.get(), &GraphvizDotTheme::default());
        Some(dot_src_and_styles)
    });

    view! {
        <DotSvg
            info_graph=progress_info_graph
            dot_src_and_styles=dot_src_and_styles
        />
    }
}

#[component]
pub fn OutcomeGraph(outcome_info_graph: Signal<InfoGraph>) -> impl IntoView {
    let dot_src_and_styles = Signal::from(move || {
        let dot_src_and_styles =
            IntoGraphvizDotSrc::into(&outcome_info_graph.get(), &GraphvizDotTheme::default());
        Some(dot_src_and_styles)
    });

    view! {
        <DotSvg
            info_graph=outcome_info_graph.into()
            dot_src_and_styles=dot_src_and_styles
        />
    }
}
