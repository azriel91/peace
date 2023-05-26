use std::path::PathBuf;

#[cfg(feature = "error_reporting")]
use peace::miette::{self, SourceSpan};

/// Error while managing a file download.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum FileDownloadError {
    #[error("Failed to open destination file.")]
    DestFileOpen(#[source] std::io::Error),
    #[error("Failed to read destination file metadata.")]
    DestMetadataRead(#[source] std::io::Error),
    #[error("Failed to read destination file contents.")]
    DestFileRead(#[source] std::io::Error),

    #[error("Failed to create directories: `{}`.", dest_parent.display())]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_item_file_download::dest_parent_dirs_create),
            help(
                "Ensure that `{}` is not a file, or rerun the command with a different path.",
                dest_parent.display())),
    )]
    DestParentDirsCreate {
        /// Destination file path.
        dest: PathBuf,
        /// Destination parent directory path.
        dest_parent: PathBuf,
        /// String representation of the destination path.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        dest_display: String,
        #[cfg(feature = "error_reporting")]
        #[label]
        parent_dirs_span: SourceSpan,
        /// Underlying IO error
        #[source]
        error: std::io::Error,
    },
    #[error("Failed to open `{}` for writing.", dest.display())]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_item_file_download::dest_file_create),
            help(
                "Ensure that `{}` is not a directory, or rerun the command with a different path.",
                dest.display())),
    )]
    DestFileCreate {
        /// Approximation of the init command that defined the destination path.
        #[cfg_attr(feature = "error_reporting", source_code)]
        init_command_approx: String,
        /// Span of the destination path within the init command.
        #[cfg(feature = "error_reporting")]
        #[label = "defined here"]
        dest_span: SourceSpan,
        /// Destination file path.
        dest: PathBuf,
        /// Underlying IO error
        #[source]
        error: std::io::Error,
    },
    #[error("Failed to delete destination file.")]
    DestFileRemove(#[source] std::io::Error),
    #[error("Failed to parse source URL.")]
    SrcUrlParse(url::ParseError),
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help(
            "Check that the URL is reachable:\n\
            {}\n\
            Make sure you are connected to the internet and try again.",
            src
        ))
    )]
    #[error("Failed to download file.")]
    SrcGet {
        /// Source URL.
        src: url::Url,
        /// Underlying error.
        #[source]
        error: reqwest::Error,
    },
    #[error("Failed to fetch source file metadata. Response status code: {status_code}")]
    SrcFileUndetermined { status_code: reqwest::StatusCode },
    #[error("Failed to read source file content.")]
    SrcFileRead(#[source] reqwest::Error),
    #[error("Failed to stream source file content.")]
    ResponseBytesStream(#[source] reqwest::Error),
    #[error("Failed to transfer source file content.")]
    ResponseFileWrite(#[source] std::io::Error),

    // Native errors
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Failed to read current executable path.")]
    CurrentExeRead(#[source] std::io::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Failed to get current executable name from path.")]
    CurrentExeNameRead,
    /// This one should be relatively unreachable.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Failed to format string in memory.")]
    FormatString(#[source] std::fmt::Error),

    // WASM errors.
    #[cfg(target_arch = "wasm32")]
    #[error("Failed to read bytes from response.")]
    ResponseBytesRead(#[source] reqwest::Error),
    #[cfg(target_arch = "wasm32")]
    #[error("Failed to read text from response.")]
    ResponseTextRead(#[source] reqwest::Error),

    // === Framework errors === //
    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),
}

impl FileDownloadError {
    /// Returns `FileDownloadError::SrcGet` from a get request error.
    pub(crate) fn src_get(src: url::Url, error: reqwest::Error) -> Self {
        FileDownloadError::SrcGet { src, error }
    }
}
