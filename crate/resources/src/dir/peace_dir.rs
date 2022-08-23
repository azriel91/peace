use std::{
    ffi::OsStr,
    ops::Deref,
    path::{Path, PathBuf},
};

use crate::dir::WorkspaceDir;

/// Directory to store all data produced by `peace` tool execution.
///
/// Typically `$workspace_dir/.peace`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeaceDir(PathBuf);

impl PeaceDir {
    /// Default name of the `.peace` directory.
    pub const NAME: &'static str = ".peace";

    /// Returns a new [`PeaceDir`].
    ///
    /// See `PeaceDir::from<&WorkspaceDir>` if you want to construct a
    /// `PeaceDir` with the default `$workspace_dir/.peace` name.
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    /// Returns the inner [`PathBuf`].
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl From<&WorkspaceDir> for PeaceDir {
    fn from(workspace_dir: &WorkspaceDir) -> Self {
        let mut path = workspace_dir.to_path_buf();
        path.push(PeaceDir::NAME);

        Self(path)
    }
}

impl From<PathBuf> for PeaceDir {
    fn from(path_buf: PathBuf) -> Self {
        Self(path_buf)
    }
}

impl AsRef<OsStr> for PeaceDir {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl AsRef<Path> for PeaceDir {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Deref for PeaceDir {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
