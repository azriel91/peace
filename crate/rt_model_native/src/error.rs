// Remember to add common variants to `rt_model_web/src/error.rs`.

use std::{ffi::OsString, path::PathBuf, sync::Mutex};

/// Peace runtime errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to deserialize current states.
    #[error("Failed to deserialize current states.")]
    StatesCurrentDeserialize(#[source] serde_yaml::Error),
    /// Failed to serialize current states.
    #[error("Failed to serialize current states.")]
    StatesCurrentSerialize(#[source] serde_yaml::Error),
    /// Current states have not been written to disk.
    ///
    /// This is returned when `StatesCurrentFile` is attempted to be
    /// deserialized but does not exist.
    #[error("Current states have not been written to disk.")]
    StatesCurrentDiscoverRequired,

    /// Failed to deserialize desired states.
    #[error("Failed to deserialize desired states.")]
    StatesDesiredDeserialize(#[source] serde_yaml::Error),
    /// Failed to serialize desired states.
    #[error("Failed to serialize desired states.")]
    StatesDesiredSerialize(#[source] serde_yaml::Error),
    /// Desired states have not been written to disk.
    ///
    /// This is returned when `StatesDesiredFile` is attempted to be
    /// deserialized but does not exist.
    #[error("Desired states have not been written to disk.")]
    StatesDesiredDiscoverRequired,

    /// Failed to serialize state diffs.
    #[error("Failed to serialize state diffs.")]
    StateDiffsSerialize(#[source] serde_yaml::Error),

    // Native errors.
    /// Failed to create file for writing.
    #[error("Failed to create file for writing: `{path}`")]
    FileCreate {
        /// Path to the file.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },
    /// Failed to open file for reading.
    #[error("Failed to open file for reading: `{path}`")]
    FileOpen {
        /// Path to the file.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },
    /// Failed to read from file.
    #[error("Failed to read from file: `{path}`")]
    FileRead {
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
    /// Failed to write to stdout.
    #[error("Failed to write to stdout.")]
    StdoutWrite(#[source] std::io::Error),
    /// Storage synchronous thread failed to be joined.
    ///
    /// This variant is used for thread spawning errors for both reads and
    /// writes.
    #[error("Storage synchronous thread failed to be joined.")]
    StorageSyncThreadSpawn(#[source] std::io::Error),
    /// Storage synchronous thread failed to be joined.
    ///
    /// This variant is used for thread spawning errors for both reads and
    /// writes.
    ///
    /// Note: The underlying thread join error does not implement
    /// `std::error::Error`. See
    /// <https://doc.rust-lang.org/std/thread/type.Result.html>.
    ///
    /// The `Mutex` is needed to allow `Error` to be `Sync`.
    #[error("Storage synchronous thread failed to be joined.")]
    StorageSyncThreadJoin(Mutex<Box<dyn std::any::Any + Send + 'static>>),
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
