use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Link, Stylesheet};
use leptos_router::{Route, Router, Routes};

use crate::FlowGraph;

/// Top level component of the `WebiOutput`.
#[component]
pub fn Home() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

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
                        <FlowGraph />
                    }/>
                </Routes>
            </main>
        </Router>
    }
}
