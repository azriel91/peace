use leptos::{component, view, IntoView, Scope};
use leptos_meta::{provide_meta_context, Link, Stylesheet};
use leptos_router::{Route, Router, Routes};

use crate::web::components::FlowGraph;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="tailwind" href="/pkg/envman.css"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=move |cx| view! {
                        cx,
                        <FlowGraph />
                    }/>
                </Routes>
            </main>
        </Router>
    }
}
