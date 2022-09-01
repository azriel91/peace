// Remember to add common variants to `rt_model_web/src/error.rs`.

use std::{ffi::OsString, path::PathBuf, sync::Mutex};

/// Peace runtime errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to serialize states.
    #[error("Failed to serialize states.")]
    StatesSerialize(#[source] serde_yaml::Error),
    /// Failed to serialize desired states.
    #[error("Failed to serialize desired states.")]
    StatesDesiredSerialize(#[source] serde_yaml::Error),

    // Native FS errors.
    /// Failed to open file for writing.
    #[error("Failed to open file for writing: `{path}`")]
    FileCreate {
        /// Path to the file.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },
    /// Failed to write to file.
    #[error("Failed to write to file: `{path}`")]
    FileWrite {
        /// Path to the file.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },
    /// File write thread failed to be joined.
    #[error("File write thread failed to be joined.")]
    FileWriteThreadSpawn(#[source] std::io::Error),
    /// File write thread failed to be joined.
    ///
    /// Note: The underlying thread join error does not implement
    /// `std::error::Error`. See
    /// <https://doc.rust-lang.org/std/thread/type.Result.html>.
    ///
    /// The `Mutex` is needed to allow `Error` to be `Sync`.
    #[error("File write thread failed to be joined.")]
    FileWriteThreadJoin(Mutex<Box<dyn std::any::Any + Send + 'static>>),
    /// Failed to read current directory to discover workspace directory.
    #[error("Failed to read current directory to discover workspace directory.")]
    WorkingDirRead(#[source] std::io::Error),
    /// Failed to create a workspace directory.
    #[error("Failed to create workspace directory: `{path}`.", path = path.display())]
    WorkspaceDirCreate {
        /// The directory that was attempted to be created.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },
    /// Failed to determine workspace directory.
    #[error(
        "Failed to determine workspace directory as could not find `{file_name}` \
            in `{working_dir}` or any parent directories.",
        file_name = file_name.to_string_lossy(),
        working_dir = working_dir.display())]
    WorkspaceFileNotFound {
        /// Beginning directory of traversal.
        working_dir: PathBuf,
        /// File or directory name searched for.
        file_name: OsString,
    },
}
