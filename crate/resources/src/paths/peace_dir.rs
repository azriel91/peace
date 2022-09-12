use std::path::PathBuf;

use crate::paths::WorkspaceDir;

/// Directory to store all data produced by `peace` tool execution.
///
/// Typically `$workspace_dir/.peace`.
///
/// See `PeaceDir::from<&WorkspaceDir>` if you want to construct a `PeaceDir`
/// with the conventional `$workspace_dir/.peace` path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeaceDir(PathBuf);

crate::paths::pathbuf_newtype!(PeaceDir);

impl PeaceDir {
    /// Default name of the `.peace` directory.
    pub const NAME: &'static str = ".peace";
}

impl From<&WorkspaceDir> for PeaceDir {
    fn from(workspace_dir: &WorkspaceDir) -> Self {
        let path = workspace_dir.join(PeaceDir::NAME);

        Self(path)
    }
}
