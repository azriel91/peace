use std::{net::SocketAddr, path::PathBuf};

use axum::Router;
use leptos::view;
use leptos_axum::LeptosRoutes;
use tokio::io::AsyncWriteExt;
use tower_http::services::ServeDir;

use crate::{model::EnvManError, web::components::Home};

/// Web server that responds to `envman` requests.
#[derive(Debug)]
pub struct WebServer {}

impl WebServer {
    /// Starts the web server.
    pub async fn start(socket_addr: Option<SocketAddr>) -> Result<(), EnvManError> {
        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = leptos::get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let socket_addr = socket_addr.unwrap_or(leptos_options.site_addr);
        let routes = leptos_axum::generate_route_list(|| view! {  <Home /> });

        let app = Router::new()
            // serve the pkg directory
            .nest_service(
                "/pkg",
                ServeDir::new(PathBuf::from_iter([
                    leptos_options.site_root.as_str(),
                    leptos_options.site_pkg_dir.as_str(),
                ])),
            )
            // serve the SSR rendered homepage
            .leptos_routes(&leptos_options, routes, move || view! {  <Home /> })
            .with_state(leptos_options);

        let (Ok(()) | Err(_)) = tokio::io::stderr()
            .write_all(format!("listening on http://{}\n", socket_addr).as_bytes())
            .await;
        let (Ok(()) | Err(_)) = tokio::io::stderr()
            .write_all(
                format!(
                    "working dir: {}\n",
                    std::env::current_dir().unwrap().display()
                )
                .as_bytes(),
            )
            .await;
        axum::Server::bind(&socket_addr)
            .serve(app.into_make_service())
            .await
            .map_err(|error| EnvManError::WebServerServe { error })
    }
}
