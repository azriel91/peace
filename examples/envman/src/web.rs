//! Runs `envman` as a web application.
//!
//! See <https://leptos-rs.github.io/leptos> for the leptos usage guide.

pub use self::flow_dot_renderer::FlowDotRenderer;

pub mod components;

mod flow_dot_renderer;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub use self::web_server::WebServer;

        mod web_server;
    } else if #[cfg(feature = "csr")] {
        pub mod client;
    }
}
