use std::{fmt::Debug, net::SocketAddr, path::Path};

use axum::Router;
use futures::stream::{self, StreamExt, TryStreamExt};
use leptos::view;
use leptos_axum::LeptosRoutes;
use peace_flow_model::FlowSpecInfo;
use peace_webi_components::Home;
use peace_webi_model::WebiError;
use tokio::io::AsyncWriteExt;
use tower_http::services::ServeDir;

/// An `OutputWrite` implementation that writes to web elements.
#[derive(Clone, Debug)]
pub struct WebiServer {
    /// IP address and port to listen on.
    socket_addr: Option<SocketAddr>,
    /// Flow to display to the user.
    flow_spec_info: FlowSpecInfo,
}

impl WebiServer {
    pub fn new(socket_addr: Option<SocketAddr>, flow_spec_info: FlowSpecInfo) -> Self {
        Self {
            socket_addr,
            flow_spec_info,
        }
    }
}

impl WebiServer {
    pub async fn start(&mut self) -> Result<(), WebiError> {
        let Self {
            socket_addr,
            flow_spec_info,
        } = self;

        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = leptos::get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let socket_addr = socket_addr.unwrap_or(leptos_options.site_addr);
        let routes = leptos_axum::generate_route_list(move || view! {  <Home /> });

        stream::iter(crate::assets::ASSETS.into_iter())
            .map(Result::<_, WebiError>::Ok)
            .try_for_each(|(path_str, contents)| async move {
                let asset_path = Path::new(path_str);
                if let Some(parent_dir) = asset_path.parent() {
                    tokio::fs::create_dir_all(parent_dir)
                        .await
                        .map_err(|error| WebiError::AssetDirCreate {
                            asset_dir: parent_dir.to_path_buf(),
                            error,
                        })?;
                }

                tokio::fs::write(asset_path, contents)
                    .await
                    .map_err(|error| WebiError::AssetWrite {
                        asset_path: asset_path.to_path_buf(),
                        error,
                    })?;

                Ok(())
            })
            .await?;

        let flow_spec_info = flow_spec_info.clone();
        let router = Router::new()
            // serve the pkg directory
            .nest_service(
                "/pkg",
                ServeDir::new(Path::new(leptos_options.site_pkg_dir.as_str())),
            )
            // serve the `webi` directory
            .nest_service("/webi", ServeDir::new(Path::new("webi")))
            // serve the SSR rendered homepage
            .leptos_routes_with_context(
                &leptos_options,
                routes,
                move || leptos::provide_context(flow_spec_info.clone()),
                move || view! {  <Home /> },
            )
            .with_state(leptos_options);

        let listener = tokio::net::TcpListener::bind(socket_addr)
            .await
            .unwrap_or_else(|e| panic!("Failed to listen on {socket_addr}. Error: {e}"));
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
        axum::serve(listener, router)
            .await
            .map_err(|error| WebiError::ServerServe { socket_addr, error })
    }
}
