use std::{net::SocketAddr, path::PathBuf};

/// Errors concerning the web interface.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum WebiError {
    /// Failed to create asset directory.
    #[error("Failed to create asset directory: `{asset_dir}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_webi_model::webi_asset_dir_create),
            help("Check if you have sufficient permission to write to the directory.")
        )
    )]
    AssetDirCreate {
        /// The directory attempted to be created.
        asset_dir: PathBuf,
        /// The underlying error.
        error: std::io::Error,
    },

    /// Failed to create asset directory.
    #[error("Failed to write asset: `{asset_path}`")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_webi_model::webi_asset_write),
            help("Check if you have sufficient permission to write to the file.")
        )
    )]
    AssetWrite {
        /// Path to the file attempted to be written to.
        asset_path: PathBuf,
        /// The underlying error.
        error: std::io::Error,
    },

    /// Failed to read leptos configuration.
    ///
    /// This may be a bug in the Peace framework or `leptos`.
    #[error("Failed to read leptos configuration. This may be a bug in the Peace framework or `leptos`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_webi_model::leptos_config_read),
            help("Ask for help on Discord.")
        )
    )]
    LeptosConfigRead {
        /// The underlying error.
        error: leptos_config::errors::LeptosConfigError,
    },

    /// Failed to start web server for Web interface.
    #[error("Failed to start web server for Web interface on socket: {socket_addr}")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_webi_model::webi_axum_serve),
            help("Another process may be using the same socket address.")
        )
    )]
    ServerServe {
        /// The socket address that the web server attempted to listen on.
        socket_addr: SocketAddr,
        /// Underlying error.
        #[source]
        error: std::io::Error,
    },
}
