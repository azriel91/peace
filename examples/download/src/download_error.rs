#[cfg(feature = "error_reporting")]
use peace::miette;

/// Error while managing a file download.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    /// A `FileDownload` item spec  error occurred.
    #[error("A `FileDownload` item spec  error occurred.")]
    PeaceItemSpecFileDownload(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace_item_spec_file_download::FileDownloadError,
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
