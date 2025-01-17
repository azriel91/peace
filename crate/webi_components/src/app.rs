use std::time::Duration;

use leptos::{
    component,
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view, IntoView,
};
use leptos_meta::{provide_meta_context, Link, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes, RoutingProgress},
    StaticSegment,
};

use crate::ChildrenFn;

/// Top level component of the `WebiOutput`.
///
/// # Parameters:
///
/// * `flow_component`: The web component to render for the flow.
#[component]
pub fn App(app_home: ChildrenFn) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let site_prefix = option_env!("SITE_PREFIX").unwrap_or("");
    let favicon_path = format!("{site_prefix}/webi/favicon.ico");
    let fonts_path = format!("{site_prefix}/webi/fonts/fonts.css");

    let (is_routing, set_is_routing) = signal(false);

    view! {
        <Link rel="shortcut icon" type_="image/ico" href=favicon_path />
        <Stylesheet id="fonts" href=fonts_path />
        <Router set_is_routing>
            <div class="routing-progress">
                <RoutingProgress is_routing max_time=Duration::from_millis(250)/>
            </div>
            <main>
                <Routes fallback=RouterFallback>
                    <Route
                        path=StaticSegment(site_prefix)
                        view=move || app_home.call()
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn RouterFallback() -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let pathname = move || location.pathname.get();

    view! {
        <p>"Path not found: " {pathname}</p>
    }
}
