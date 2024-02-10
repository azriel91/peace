use std::{fmt::Debug, net::SocketAddr, path::PathBuf};

use axum::Router;
use leptos::view;
use leptos_axum::LeptosRoutes;
use peace_fmt::Presentable;
use peace_rt_model_core::{async_trait, output::OutputWrite};
use peace_value_traits::AppError;
use peace_webi_components::Home;
use peace_webi_model::WebiError;
use tokio::io::AsyncWriteExt;
use tower_http::services::ServeDir;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_core::progress::{
            // ProgressComplete,
            // ProgressLimit,
            // ProgressStatus,
            ProgressTracker,
            // ProgressUpdate,
            ProgressUpdateAndId,
        };
        use peace_rt_model_core::CmdProgressTracker;
    }
}

/// An `OutputWrite` implementation that writes to web elements.
#[derive(Debug)]
pub struct WebiOutput {
    /// IP address and port to listen on.
    socket_addr: Option<SocketAddr>,
}

impl WebiOutput {
    pub fn new(socket_addr: Option<SocketAddr>) -> Self {
        Self { socket_addr }
    }
}

impl WebiOutput {
    pub async fn start(&self) -> Result<(), WebiError> {
        let Self { socket_addr } = self;

        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = leptos::get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let socket_addr = socket_addr.unwrap_or(leptos_options.site_addr);
        let routes = leptos_axum::generate_route_list(|| view! {  <Home /> });

        let router = Router::new()
            // serve the pkg directory
            .nest_service(
                "/pkg",
                ServeDir::new(PathBuf::from(leptos_options.site_pkg_dir.as_str())),
            )
            // serve the SSR rendered homepage
            .leptos_routes(&leptos_options, routes, move || view! {  <Home /> })
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

#[async_trait(?Send)]
impl<AppErrorT> OutputWrite<AppErrorT> for WebiOutput
where
    AppErrorT: AppError,
{
    #[cfg(feature = "output_progress")]
    async fn progress_begin(&mut self, _cmd_progress_tracker: &CmdProgressTracker) {}

    #[cfg(feature = "output_progress")]
    async fn progress_update(
        &mut self,
        _progress_tracker: &ProgressTracker,
        _progress_update_and_id: &ProgressUpdateAndId,
    ) {
    }

    #[cfg(feature = "output_progress")]
    async fn progress_end(&mut self, _cmd_progress_tracker: &CmdProgressTracker) {}

    async fn present<P>(&mut self, _presentable: P) -> Result<(), AppErrorT>
    where
        AppErrorT: std::error::Error,
        P: Presentable,
    {
        todo!()
    }

    async fn write_err(&mut self, _error: &AppErrorT) -> Result<(), AppErrorT>
    where
        AppErrorT: std::error::Error,
    {
        todo!()
    }
}
