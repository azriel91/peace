use std::path::PathBuf;

use crate::paths::PeaceDir;

/// Path to the file that stores the workspace initialization parameters.
///
/// Typically `$workspace_dir/.peace/init.yaml`.
///
/// See `WorkspaceParamsFile::from<&PeaceDir>` if you want to construct a
/// `WorkspaceParamsFile` with the conventional `$peace_dir/init.yaml`
/// path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceParamsFile(PathBuf);

crate::paths::pathbuf_newtype!(WorkspaceParamsFile);

impl WorkspaceParamsFile {
    /// File name of the initialization parameters file.
    pub const NAME: &'static str = "init.yaml";
}

impl From<&PeaceDir> for WorkspaceParamsFile {
    fn from(flow_dir: &PeaceDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
