#[cfg(feature = "error_reporting")]
use peace::miette;

/// Error while managing a web application.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum WebAppError {
    /// Failed to construct web application download URL.
    #[error("Failed to construct web application download URL.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(web_app::web_app_url_build),
            help("If the URL is valid, this may be a bug in the example, or the `url` library.")
        )
    )]
    WebAppUrlBuild {
        /// Computed URL that is attempted to be parsed.
        #[cfg_attr(feature = "error_reporting", source_code)]
        url_candidate: String,
        /// URL parse error.
        #[source]
        error: url::ParseError,
    },
    /// Failed to parse environment type.
    #[error("Failed to parse environment type.")]
    EnvTypeParseError(
        #[source]
        #[from]
        crate::model::EnvTypeParseError,
    ),

    // === Item Spec errors === //
    /// A `FileDownload` item spec error occurred.
    #[error("A `FileDownload` item spec error occurred.")]
    PeaceItemSpecFileDownload(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace_item_specs::file_download::FileDownloadError,
    ),

    // === Framework errors === //
    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),

    // === Scaffolding errors === //
    #[error("Failed to initialize tokio runtime.")]
    TokioRuntimeInit(#[source] std::io::Error),
}
