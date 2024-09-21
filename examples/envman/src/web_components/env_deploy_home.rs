use leptos::{component, view, IntoView, SignalSet};
use peace::webi_components::FlowGraph;

use crate::flows::EnvDeployFlow;

/// Top level component of the `WebiOutput`.
#[component]
pub fn EnvDeployHome() -> impl IntoView {
    // TODO: allow users to select which flow they want.
    let (flow, flow_set) = leptos::create_signal(None);
    let _flow_resource = leptos::create_resource(
        || (),
        move |()| async move {
            let flow = EnvDeployFlow::flow().await.ok();
            flow_set.set(flow);
        },
    );

    view! {
        <FlowGraph flow />
    }
}
