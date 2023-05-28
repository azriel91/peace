use std::net::SocketAddr;

use dioxus::prelude::*;

/// Parameters for the homepage.
#[derive(PartialEq, Props)]
pub struct HomeProps {
    pub socket_addr: SocketAddr,
}

pub fn Home(cx: Scope<HomeProps>) -> Element {
    let socket_addr = &cx.props.socket_addr;
    cx.render(rsx!(
        head {
            link {
                href: "/public/css/tailwind.css",
                rel: "stylesheet",
            }
        }
        body {
            div {
                id: "main",
            }
            dioxus_liveview::interpreter_glue(&format!("ws://{socket_addr}/ws"))
        }
    ))
}
