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
            code(peace_item_spec_file_download::dest_parent_dirs_create),
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
            code(peace_item_spec_file_download::dest_file_create),
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
        diagnostic(
            code(peace_item_spec_file_download::src_get),
            help(
                "Check that the URL is reachable: `curl {}`\nAre you connected to the internet?",
                src
            ),
        )
    )]
    #[error("Failed to download file.")]
    SrcGet {
        /// Approximation of the init command that defined the source URL.
        #[cfg(not(target_arch = "wasm32"))]
        #[cfg_attr(feature = "error_reporting", source_code)]
        init_command_approx: String,
        /// Span of the source URL within the init command.
        #[cfg(not(target_arch = "wasm32"))]
        #[cfg(feature = "error_reporting")]
        #[label = "defined here"]
        src_span: SourceSpan,
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
    #[error("Failed to read text from contents.")]
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
    ///
    /// One of the other variants may be returned if failing to construct the
    /// `SrcGet` error:
    ///
    /// * `CurrentExeRead`: If the OS does not return the current executable
    ///   path.
    /// * `CurrentExeNameRead`: If the current executable path is not a
    ///   [`Normal`] path component.
    /// * `FormatString`: If formatting a string fails, maybe running out of
    ///   memory?
    ///
    /// [`Normal`]: std::path::Component::Normal
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn src_get(
        src: url::Url,
        dest: &std::path::Path,
        error: reqwest::Error,
    ) -> Result<Self, Self> {
        use std::{fmt::Write, path::Component};

        #[cfg(feature = "error_reporting")]
        use peace::miette::SourceOffset;

        let mut init_command_approx = String::with_capacity(256);
        let exe_path = std::env::current_exe().map_err(FileDownloadError::CurrentExeRead)?;
        let exe_name = if let Some(Component::Normal(exe_name)) = exe_path.components().next_back()
        {
            exe_name
        } else {
            return Err(FileDownloadError::CurrentExeNameRead);
        };

        let exe_name = exe_name.to_string_lossy();
        let dest_display = dest.display();

        write!(&mut init_command_approx, "{exe_name} init ")
            .map_err(FileDownloadError::FormatString)?;
        #[cfg(feature = "error_reporting")]
        let src_offset_col = init_command_approx.len();
        write!(&mut init_command_approx, "{src}").map_err(FileDownloadError::FormatString)?;
        #[cfg(feature = "error_reporting")]
        let dest_offset_col = init_command_approx.len();
        write!(&mut init_command_approx, " {dest_display}")
            .map_err(FileDownloadError::FormatString)?;

        #[cfg(feature = "error_reporting")]
        let src_span = {
            let loc_line = 1;
            // Add one to offset because we are 1-based, not 0-based?
            let start =
                SourceOffset::from_location(&init_command_approx, loc_line, src_offset_col + 1);
            // Add one to length because we are 1-based, not 0-based?
            let length = SourceOffset::from_location(
                &init_command_approx,
                loc_line,
                dest_offset_col - src_offset_col + 1,
            );
            SourceSpan::new(start, length)
        };
        Err(FileDownloadError::SrcGet {
            init_command_approx,
            #[cfg(feature = "error_reporting")]
            src_span,
            src,
            error,
        })
    }

    /// Returns `FileDownloadError::SrcGet` from a get request error.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn src_get(src: url::Url, error: reqwest::Error) -> Self {
        FileDownloadError::SrcGet { src, error }
    }
}
