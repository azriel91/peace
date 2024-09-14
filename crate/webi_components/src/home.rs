use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Link, Stylesheet};
use leptos_router::{Route, Router, Routes};
use peace_rt_model::Flow;

use crate::FlowGraph;

/// Top level component of the `WebiOutput`.
///
/// # Parameters:
///
/// * `flow`: The flow available to the web UI.
#[component]
pub fn Home<E>(flow: Flow<E>) -> impl IntoView
where
    E: 'static,
{
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // TODO: when multiple flows are supported, set flow.
    let (flow, _flow_set) = leptos::create_signal(flow);

    let site_prefix = option_env!("SITE_PREFIX").unwrap_or("");
    let favicon_path = format!("{site_prefix}/webi/favicon.ico");
    let fonts_path = format!("{site_prefix}/webi/fonts/fonts.css");

    view! {
        <Link rel="shortcut icon" type_="image/ico" href=favicon_path />
        <Stylesheet id="fonts" href=fonts_path />
        <Router>
            <main>
                <Routes>
                    <Route path=site_prefix view=move || view! {
                        <FlowGraph flow />
                    }/>
                </Routes>
            </main>
        </Router>
    }
}
