use std::net::{IpAddr, SocketAddr};

use axum::{response::Html, routing::get, Router};
use dioxus::prelude::VirtualDom;
use tower_http::services::ServeDir;

use crate::{model::EnvManError, web::components::Home};

/// Web server that responds to `envman` requests.
#[derive(Debug)]
pub struct WebServer {}

/// See <https://github.com/DioxusLabs/example-projects/blob/master/ecommerce-site/src/main.rs>
/// for referenced code.

impl WebServer {
    /// Starts the web server.
    pub async fn start(ip_addr: IpAddr, port: u16) -> Result<(), EnvManError> {
        let addr = SocketAddr::from((ip_addr, port));

        let app = Router::new()
            // serve the public directory
            .nest_service("/public", ServeDir::new("public"))
            // serve the SSR rendered homepage
            .route("/", get(root));

        println!("listening on http://{}", addr);
        println!("- Route available on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .map_err(|error| EnvManError::WebServerServe { error })
    }
}

// Just render a simple page directly from the request
async fn root() -> Html<String> {
    // The root page blocks on futures so we need to render it in a spawn_blocking
    // task
    tokio::task::spawn_blocking(move || async move {
        let mut app = VirtualDom::new(Home);
        let _ = app.rebuild();
        Html(dioxus_ssr::render(&app))
    })
    .await
    .unwrap()
    .await
}
