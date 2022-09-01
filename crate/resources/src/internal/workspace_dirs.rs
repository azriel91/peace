use crate::dir::{FlowDir, PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir};

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
    /// Directory to store data for the current profile.
    profile_dir: ProfileDir,
    /// Directory to store profile executions' summaries.
    profile_history_dir: ProfileHistoryDir,
    /// Directory to store data for the current flow.
    flow_dir: FlowDir,
}

impl WorkspaceDirs {
    /// Returns new `WorkspaceDirs`.
    pub fn new(
        workspace_dir: WorkspaceDir,
        peace_dir: PeaceDir,
        profile_dir: ProfileDir,
        profile_history_dir: ProfileHistoryDir,
        flow_dir: FlowDir,
    ) -> Self {
        Self {
            workspace_dir,
            peace_dir,
            profile_dir,
            profile_history_dir,
            flow_dir,
        }
    }

    /// Returns the individual workspace directories.
    pub fn into_inner(
        self,
    ) -> (
        WorkspaceDir,
        PeaceDir,
        ProfileDir,
        ProfileHistoryDir,
        FlowDir,
    ) {
        let Self {
            workspace_dir,
            peace_dir,
            profile_dir,
            profile_history_dir,
            flow_dir,
        } = self;

        (
            workspace_dir,
            peace_dir,
            profile_dir,
            profile_history_dir,
            flow_dir,
        )
    }

    /// Returns a reference to the workspace directory.
    pub fn workspace_dir(&self) -> &WorkspaceDir {
        &self.workspace_dir
    }

    /// Returns a reference to the `.peace` directory.
    pub fn peace_dir(&self) -> &PeaceDir {
        &self.peace_dir
    }

    /// Returns a reference to the profile directory.
    pub fn profile_dir(&self) -> &ProfileDir {
        &self.profile_dir
    }

    /// Returns a reference to the profile history directory.
    pub fn profile_history_dir(&self) -> &ProfileHistoryDir {
        &self.profile_history_dir
    }

    /// Returns a reference to the flow directory.
    pub fn flow_dir(&self) -> &FlowDir {
        &self.flow_dir
    }
}
