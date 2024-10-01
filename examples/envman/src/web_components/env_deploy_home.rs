use leptos::{component, view, IntoView};
use peace::webi_components::FlowGraph;

/// Top level component of the `WebiOutput`.
#[component]
pub fn EnvDeployHome() -> impl IntoView {
    view! {
        <div>
            <h1>"Environment"</h1>

            <h2>"Example"</h2>
            <FlowGraph />

            <h2>"Current"</h2>
        </div>
    }
}
