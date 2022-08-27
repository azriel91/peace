use peace_resources::dir::{PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir};

/// Directories used during `peace` execution.
///
/// This is not part of the [`peace_resources`] crate as it is not a resource,
/// but rather a grouping of directories where data is stored. This is created
/// by [`WorkspaceDirsBuilder`].
///
/// [`WorkspaceDirsBuilder`]: crate::WorkspaceDirsBuilder
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceDirs {
    /// Base directory of the workspace.
    workspace_dir: WorkspaceDir,
    /// Peace directory,
    peace_dir: PeaceDir,
    /// Directory to store data for the current profile.
    profile_dir: ProfileDir,
    /// Directory to store profile executions' summaries.
    profile_history_dir: ProfileHistoryDir,
}

impl WorkspaceDirs {
    /// Returns new `WorkspaceDirs`.
    pub fn new(
        workspace_dir: WorkspaceDir,
        peace_dir: PeaceDir,
        profile_dir: ProfileDir,
        profile_history_dir: ProfileHistoryDir,
    ) -> Self {
        Self {
            workspace_dir,
            peace_dir,
            profile_dir,
            profile_history_dir,
        }
    }

    /// Returns the individual workspace directories.
    pub fn into_inner(self) -> (WorkspaceDir, PeaceDir, ProfileDir, ProfileHistoryDir) {
        let Self {
            workspace_dir,
            peace_dir,
            profile_dir,
            profile_history_dir,
        } = self;

        (workspace_dir, peace_dir, profile_dir, profile_history_dir)
    }

    /// Returns a reference to the workspace dir.
    pub fn workspace_dir(&self) -> &WorkspaceDir {
        &self.workspace_dir
    }

    /// Returns a reference to the `.peace` dir.
    pub fn peace_dir(&self) -> &PeaceDir {
        &self.peace_dir
    }

    /// Returns a reference to the profile dir.
    pub fn profile_dir(&self) -> &ProfileDir {
        &self.profile_dir
    }

    /// Returns a reference to the profile history dir.
    pub fn profile_history_dir(&self) -> &ProfileHistoryDir {
        &self.profile_history_dir
    }
}
