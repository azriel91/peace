use crate::paths::{PeaceAppDir, PeaceDir, WorkspaceDir};

/// Directories used during `peace` execution.
///
/// This type itself is not inserted into `Resources`, but each of the member
/// directories are individually inserted. This is created by
/// `WorkspaceDirsBuilder` from either the `peace_rt_model` or
/// `peace_rt_model_web` crates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceDirs {
    /// Base directory of the workspace.
    workspace_dir: WorkspaceDir,
    /// Peace directory,
    peace_dir: PeaceDir,
    /// Peace app directory,
    peace_app_dir: PeaceAppDir,
}

impl WorkspaceDirs {
    /// Returns new `WorkspaceDirs`.
    pub fn new(
        workspace_dir: WorkspaceDir,
        peace_dir: PeaceDir,
        peace_app_dir: PeaceAppDir,
    ) -> Self {
        Self {
            workspace_dir,
            peace_dir,
            peace_app_dir,
        }
    }

    /// Returns the individual workspace directories.
    pub fn into_inner(self) -> (WorkspaceDir, PeaceDir, PeaceAppDir) {
        let Self {
            workspace_dir,
            peace_dir,
            peace_app_dir,
        } = self;

        (workspace_dir, peace_dir, peace_app_dir)
    }

    /// Returns a reference to the workspace directory.
    pub fn workspace_dir(&self) -> &WorkspaceDir {
        &self.workspace_dir
    }

    /// Returns a reference to the `.peace` directory.
    pub fn peace_dir(&self) -> &PeaceDir {
        &self.peace_dir
    }

    /// Returns a reference to the `.peace/$app` directory.
    pub fn peace_app_dir(&self) -> &PeaceAppDir {
        &self.peace_app_dir
    }
}
