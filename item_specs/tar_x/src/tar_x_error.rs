use std::path::PathBuf;

#[cfg(feature = "error_reporting")]
use peace::miette;

/// Error while managing tar extraction.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum TarXError {
    /// Tar file to extract doesn't exist.
    #[error(
        r#"Tar file to extract doesn't exist: `{}`"#,
        tar_path.display()
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_tar_x::tar_file_non_existent))
    )]
    TarFileNonExistent {
        /// Path to the tar file to extract.
        tar_path: PathBuf,
    },

    /// Failed to read tar entries.
    #[error(
        r#"Failed to read tar entries: `{}`"#,
        tar_path.display()
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_tar_x::tar_entry_read))
    )]
    TarEntryRead {
        /// Path to the tar file.
        tar_path: PathBuf,
        /// Underlying error.
        error: std::io::Error,
    },

    /// Failed to read tar entry path.
    #[error(
        r#"Failed to read tar entry path: `{}`"#,
        tar_path.display()
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_tar_x::tar_entry_path_read))
    )]
    TarEntryPathRead {
        /// Path to the tar file.
        tar_path: PathBuf,
        /// Underlying error.
        error: std::io::Error,
    },

    /// Failed to read tar entry modified time.
    #[error(
        r#"Failed to read tar entry modified time: `{}`"#,
        tar_path.display()
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_tar_x::tar_entry_m_time_non_existent))
    )]
    TarEntryMTimeRead {
        /// Path to the tar file.
        tar_path: PathBuf,
        /// Entry path in the tar file.
        entry_path: PathBuf,
        /// Underlying error.
        error: std::io::Error,
    },

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
