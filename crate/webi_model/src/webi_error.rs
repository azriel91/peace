use std::net::SocketAddr;

/// Errors concerning the web interface.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum WebiError {
    /// Failed to start web server for Web interface.
    #[error("Failed to start web server for Web interface on socket: {socket_addr}")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_webi_model::webi_axum_serve),
            help("Another process may be using the same socket address: {socket_addr}.")
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
