use std::path::PathBuf;

use crate::paths::PeaceAppDir;

/// Path to the file that stores the workspace initialization parameters.
///
/// Typically `$workspace_dir/.peace/$app/workspace_params.yaml`.
///
/// See `WorkspaceParamsFile::from<&PeaceAppDir>` if you want to construct a
/// `WorkspaceParamsFile` with the conventional
/// `$peace_dir/$app/workspace_params.yaml` path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceParamsFile(PathBuf);

crate::paths::pathbuf_newtype!(WorkspaceParamsFile);

impl WorkspaceParamsFile {
    /// File name of the workspace parameters file.
    pub const NAME: &'static str = "workspace_params.yaml";
}

impl From<&PeaceAppDir> for WorkspaceParamsFile {
    fn from(flow_dir: &PeaceAppDir) -> Self {
        let path = flow_dir.join(Self::NAME);

        Self(path)
    }
}
