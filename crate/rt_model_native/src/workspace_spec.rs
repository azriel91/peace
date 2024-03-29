use std::{ffi::OsString, path::PathBuf};

/// Describes how to discover the workspace directory.
#[derive(Clone, Debug, PartialEq, Eq)]
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
