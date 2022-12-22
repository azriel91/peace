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
        diagnostic(code(peace_item_spec_tar_x::tar_file_not_exists)),
        help("Make sure there is an item spec that downloads the tar file.")
    )]
    TarFileNotExists {
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
        diagnostic(code(peace_item_spec_tar_x::tar_entry_m_time_read))
    )]
    TarEntryMTimeRead {
        /// Path to the tar file.
        tar_path: PathBuf,
        /// Entry path in the tar file.
        entry_path: PathBuf,
        /// Underlying error.
        error: std::io::Error,
    },

    /// Failed to read tar extraction destination path
    #[error(
        r#"Failed to read directory within tar extraction destination path: `{}`"#,
        dir.display()
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_tar_x::tar_dest_read_dir))
    )]
    TarDestReadDir {
        /// Path within the extraction directory.
        dir: PathBuf,
        /// Underlying error.
        error: std::io::Error,
    },

    /// Failed to read destination file entry.
    #[error(
        r#"Failed to read destination file entry in `{}`"#,
        dest.display()
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_tar_x::tar_dest_entry_read))
    )]
    TarDestEntryRead {
        /// Path to the destination directory.
        dest: PathBuf,
        /// Underlying error.
        error: std::io::Error,
    },

    /// Failed to read destination file type.
    #[error(
        r#"Failed to read destination file type for `{}`"#,
        entry_path.display()
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_tar_x::tar_dest_entry_file_type_read))
    )]
    TarDestEntryFileTypeRead {
        /// Path to the file in the destination directory.
        entry_path: PathBuf,
        /// Underlying error.
        error: std::io::Error,
    },

    /// Failed to read destination file modified time.
    #[cfg(not(target_arch = "wasm32"))]
    #[error(
        r#"Failed to read destination file modified time: `{}` in `{}`"#,
        entry_path.display(),
        dest.display()
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_tar_x::tar_entry_m_time_read))
    )]
    TarDestFileMTimeRead {
        /// Path to the destination directory.
        dest: PathBuf,
        /// Entry path in the tar file.
        entry_path: PathBuf,
        /// Underlying error.
        error: std::io::Error,
    },

    /// Failed to read destination file modified time system time.
    #[cfg(not(target_arch = "wasm32"))]
    #[error(
        r#"Failed to read destination file modified time system time: `{}` in `{}`"#,
        entry_path.display(),
        dest.display()
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_tar_x::tar_dest_file_m_time_system_time_read))
    )]
    TarDestFileMTimeSystemTimeRead {
        /// Path to the destination directory.
        dest: PathBuf,
        /// Entry path in the tar file.
        entry_path: PathBuf,
        /// Underlying error.
        error: std::time::SystemTimeError,
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
