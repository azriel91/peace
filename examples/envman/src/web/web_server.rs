use std::net::{IpAddr, SocketAddr};

use axum::{extract::WebSocketUpgrade, response::Html, routing::get, Router};
use dioxus::prelude::VirtualDom;
use tokio::io::AsyncWriteExt;
use tower_http::services::ServeDir;

use crate::{
    flows::AppUploadFlow,
    model::EnvManError,
    web::components::{FlowGraph, FlowGraphProps, Home, HomeProps},
};

/// Web server that responds to `envman` requests.
#[derive(Debug)]
pub struct WebServer {}

/// See <https://github.com/DioxusLabs/example-projects/blob/master/ecommerce-site/src/main.rs>
/// for referenced code.

impl WebServer {
    /// Starts the web server.
    pub async fn start(ip_addr: IpAddr, port: u16) -> Result<(), EnvManError> {
        let socket_addr = SocketAddr::from((ip_addr, port));

        let view = dioxus_liveview::LiveViewPool::new();

        let app = Router::new()
            // serve the public directory
            .nest_service("/public", ServeDir::new("public"))
            // serve the SSR rendered homepage
            .route("/", get(move || root(socket_addr)))
            // The WebSocket route is what Dioxus uses to communicate with the browser
            .route(
                "/ws",
                get(move |ws: WebSocketUpgrade| async move {
                    ws.on_upgrade(move |socket| async move {
                        let flow = AppUploadFlow::flow().await.unwrap();

                        // When the WebSocket is upgraded, launch the LiveView with the app
                        // component
                        _ = view
                            .launch_with_props(
                                dioxus_liveview::axum_socket(socket),
                                FlowGraph,
                                FlowGraphProps { flow },
                            )
                            .await;
                    })
                }),
            );

        let (Ok(()) | Err(_)) = tokio::io::stderr()
            .write_all(format!("listening on http://{}", socket_addr).as_bytes())
            .await;
        axum::Server::bind(&socket_addr)
            .serve(app.into_make_service())
            .await
            .map_err(|error| EnvManError::WebServerServe { error })
    }
}

/// Renders the home page directly from the request
///
/// The `Home` page includes the glue code to connect to the WebSocket on the
/// `"/ws"` route.
async fn root(socket_addr: SocketAddr) -> Html<String> {
    // The root page blocks on futures so we need to render it in a spawn_blocking
    // task
    tokio::task::spawn_blocking(move || async move {
        let mut app = VirtualDom::new_with_props(Home, HomeProps { socket_addr });
        let _ = app.rebuild();
        Html(format!(
            r#"
            <!DOCTYPE html>
            <html>
                {home}
            </html>
            "#,
            home = dioxus_ssr::render(&app)
        ))
    })
    .await
    .map_err(|error| EnvManError::WebServerRenderJoin { error })
    .expect("Expected web server render thread to join successfully.")
    .await
}
