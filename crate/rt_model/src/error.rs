#[cfg(not(target_arch = "wasm32"))]
use std::{ffi::OsString, path::PathBuf, sync::Mutex};

/// Peace runtime errors.
#[derive(Debug, thiserror::Error)]
#[cfg(not(target_arch = "wasm32"))]
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
    StatesFileSerialize(#[source] serde_yaml::Error),
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

/// Peace runtime errors.
#[derive(Debug, thiserror::Error)]
#[cfg(target_arch = "wasm32")]
pub enum Error {
    /// Browser local storage unavailable.
    #[error("Browser local storage unavailable.")]
    LocalStorageUnavailable,
    /// Failed to get browser local storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[error("Failed to get browser local storage: `{0}`")]
    LocalStorageGet(String),
    /// Browser local storage is `None`.
    #[error("Browser local storage is none.")]
    LocalStorageNone,
    /// Browser session storage unavailable.
    #[error("Browser session storage unavailable.")]
    SessionStorageUnavailable,
    /// Failed to get browser session storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[error("Failed to get browser session storage: `{0}`")]
    SessionStorageGet(String),
    /// Browser session storage is `None`.
    #[error("Browser session storage is none.")]
    SessionStorageNone,
    /// Failed to set an item in browser storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[error("Failed to set an item in browser storage: `{0}`")]
    StorageSetItem(String),
    /// Failed to fetch browser Window object.
    #[error("Failed to fetch browser Window object.")]
    WindowNone,
}
