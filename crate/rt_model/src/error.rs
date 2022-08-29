#[cfg(not(target_arch = "wasm32"))]
use std::{ffi::OsString, path::PathBuf};

/// Peace runtime errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to read current directory to discover workspace directory.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Failed to read current directory to discover workspace directory.")]
    WorkingDirRead(#[source] std::io::Error),
    /// Failed to create a workspace directory.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Failed to create workspace directory: `{path}`.", path = path.display())]
    WorkspaceDirCreate {
        /// The directory that was attempted to be created.
        path: PathBuf,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },
    /// Failed to determine workspace directory.
    #[cfg(not(target_arch = "wasm32"))]
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

    /// Browser local storage unavailable.
    #[cfg(target_arch = "wasm32")]
    #[error("Browser local storage unavailable.")]
    LocalStorageUnavailable,
    /// Failed to get browser local storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[cfg(target_arch = "wasm32")]
    #[error("Failed to get browser local storage: `{0}`")]
    LocalStorageGet(String),
    /// Browser local storage is `None`.
    #[cfg(target_arch = "wasm32")]
    #[error("Browser local storage is none.")]
    LocalStorageNone,
    /// Browser session storage unavailable.
    #[cfg(target_arch = "wasm32")]
    #[error("Browser session storage unavailable.")]
    SessionStorageUnavailable,
    /// Failed to get browser session storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[cfg(target_arch = "wasm32")]
    #[error("Failed to get browser session storage: `{0}`")]
    SessionStorageGet(String),
    /// Browser session storage is `None`.
    #[cfg(target_arch = "wasm32")]
    #[error("Browser session storage is none.")]
    SessionStorageNone,
    /// Failed to set an item in browser storage.
    ///
    /// Note: The original `JsValue` error is converted to a `String` to allow
    /// this type to be `Send`.
    #[cfg(target_arch = "wasm32")]
    #[error("Failed to set an item in browser storage: `{0}`")]
    StorageSetItem(String),
    /// Failed to fetch browser Window object.
    #[cfg(target_arch = "wasm32")]
    #[error("Failed to fetch browser Window object.")]
    WindowNone,
}
