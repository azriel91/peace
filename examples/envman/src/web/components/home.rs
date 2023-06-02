use leptos::{component, create_signal, view, IntoView, Scope, SignalGet};
use leptos_meta::{Link, Stylesheet};
use leptos_router::{Route, Router, Routes};
use peace::rt_model::Flow;

use crate::{model::EnvManError, web::components::FlowGraph};

#[component]
pub fn Home(cx: Scope, flow: Flow<EnvManError>) -> impl IntoView {
    let (flow, _set_flow) = create_signal(cx, flow);
    view! {
        cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="tailwind" href="/pkg/envman.css"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=move |cx| view! {
                        cx,
                        <FlowGraph flow=flow.get() />
                    }/>
                </Routes>
            </main>
        </Router>
    }
}
