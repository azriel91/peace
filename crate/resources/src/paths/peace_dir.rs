use std::path::PathBuf;

use crate::paths::WorkspaceDir;

/// Directory to store all data produced by `peace` tool execution.
///
/// Typically `$workspace_dir/.peace`.
///
/// See `PeaceDir::from<&WorkspaceDir>` if you want to construct a
/// `PeaceDir` with the default `$workspace_dir/.peace` name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeaceDir(PathBuf);

crate::paths::pathbuf_newtype!(PeaceDir);

impl PeaceDir {
    /// Default name of the `.peace` directory.
    pub const NAME: &'static str = ".peace";
}

impl From<&WorkspaceDir> for PeaceDir {
    fn from(workspace_dir: &WorkspaceDir) -> Self {
        let mut path = workspace_dir.to_path_buf();
        path.push(PeaceDir::NAME);

        Self(path)
    }
}
