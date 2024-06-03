use std::path::PathBuf;

use peace_core::AppName;

use crate::paths::PeaceDir;

/// Directory to store all data produced by the current application's execution.
///
/// Typically `$workspace_dir/.peace/$app`.
///
/// This is the directory that contains all information produced and used during
/// a `peace` tool invocation. This directory layer is created to accommodate
/// different `peace` tools being run in the same workspace.
///
/// # Implementors
///
/// This type is constructed by the Peace framework when a Workspace's
/// directories are created.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeaceAppDir(PathBuf);

crate::paths::pathbuf_newtype!(PeaceAppDir);

impl From<(&PeaceDir, &AppName)> for PeaceAppDir {
    fn from((peace_dir, app_name): (&PeaceDir, &AppName)) -> Self {
        let path = peace_dir.join(app_name.as_ref());

        Self(path)
    }
}
