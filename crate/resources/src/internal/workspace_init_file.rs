use std::path::PathBuf;

use crate::paths::PeaceDir;

/// Path to the file that stores the workspace initialization parameters.
///
/// Typically `$workspace_dir/.peace/init.yaml`.
///
/// See `WorkspaceInitFile::from<&PeaceDir>` if you want to construct a
/// `WorkspaceInitFile` with the conventional `$peace_dir/init.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceInitFile(PathBuf);

crate::paths::pathbuf_newtype!(WorkspaceInitFile);

impl WorkspaceInitFile {
    /// File name of the initialization parameters file.
    pub const NAME: &'static str = "init.yaml";
}

impl From<&PeaceDir> for WorkspaceInitFile {
    fn from(flow_dir: &PeaceDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
