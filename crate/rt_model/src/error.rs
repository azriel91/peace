use std::{ffi::OsString, path::PathBuf, sync::Mutex};

/// Peace runtime errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to open states file for writing.
    #[error("Failed to open states file for writing: `{path}`")]
    StatesFileCreate {
        /// Path to the states file.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },
    /// Failed to serialize states.
    #[error("Failed to serialize states.")]
    StatesSerialize(#[source] serde_yaml::Error),
    /// Failed to write states file.
    #[error("Failed to write states file: `{path}`")]
    StatesFileWrite {
        /// Path to the states file.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },
    /// States file write thread failed to be joined.
    #[error("States file write thread failed to be joined.")]
    StatesFileWriteThreadSpawn(#[source] std::io::Error),
    /// States file write thread failed to be joined.
    ///
    /// Note: The underlying thread join error does not implement
    /// `std::error::Error`. See
    /// <https://doc.rust-lang.org/std/thread/type.Result.html>.
    ///
    /// The `Mutex` is needed to allow `Error` to be `Sync`.
    #[error("States file write thread failed to be joined.")]
    StatesFileWriteThreadJoin(Mutex<Box<dyn std::any::Any + Send + 'static>>),

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
