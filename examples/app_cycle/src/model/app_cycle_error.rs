#[cfg(feature = "error_reporting")]
use peace::miette;

use peace::rt_model::fn_graph::{Edge, WouldCycle};

/// Error while managing a web application.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum AppCycleError {
    /// Failed to construct web application download URL.
    #[error("Failed to construct web application download URL.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(app_cycle::app_cycle_url_build),
            help("If the URL is valid, this may be a bug in the example, or the `url` library.")
        )
    )]
    AppCycleUrlBuild {
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
    /// A `TarX` item spec error occurred.
    #[error("A `TarX` item spec error occurred.")]
    PeaceItemSpecTarX(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace_item_specs::tar_x::TarXError,
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

    /// A graph `WouldCycle` error occurred.
    #[error("A `peace` runtime error occurred.")]
    WouldCycleError(
        #[source]
        #[from]
        WouldCycle<Edge>,
    ),

    // === Scaffolding errors === //
    #[error("Failed to initialize tokio runtime.")]
    TokioRuntimeInit(#[source] std::io::Error),
}