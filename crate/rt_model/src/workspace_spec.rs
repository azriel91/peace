#[cfg(not(target_arch = "wasm32"))]
use std::ffi::OsString;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

/// Describes how to discover the workspace directory.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg(not(target_arch = "wasm32"))]
pub enum WorkspaceSpec {
    /// Use the exe working directory as the workspace directory.
    ///
    /// The working directory is the directory that the user ran the program in.
    ///
    /// # WASM
    ///
    /// When compiled to Web assembly (`target_arch = "wasm32"`), this variant
    /// indicates no prefix to keys within local storage.
    WorkingDir,
    /// Use a specified path.
    Path(PathBuf),
    /// Traverse up from the working directory until the given file is found.
    ///
    /// The workspace directory is the parent directory that contains a file or
    /// directory with the provided name.
    FirstDirWithFile(OsString),
}

/// Describes how to store peace automation data.
///
/// See <https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API>.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg(target_arch = "wasm32")]
pub enum WorkspaceSpec {
    /// Use browser local storage to store peace data.
    ///
    /// Persists even when the browser is closed and reopened.
    ///
    /// * Stores data with no expiration date, and gets cleared only through
    ///   JavaScript, or clearing the Browser cache / Locally Stored Data.
    /// * Storage limit is the maximum amongst the two.
    LocalStorage,
    /// Use session storage to store peace data.
    ///
    /// Maintains a separate storage area for each given origin that's available
    /// for the duration of the page session (as long as the browser is open,
    /// including page reloads and restores)
    ///
    /// * Stores data only for a session, meaning that the data is stored until
    ///   the browser (or tab) is closed.
    /// * Data is never transferred to the server.
    /// * Storage limit is larger than a cookie (at most 5MB).
    SessionStorage,
}
