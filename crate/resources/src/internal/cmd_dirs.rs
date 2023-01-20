use crate::paths::{FlowDir, ProfileDir, ProfileHistoryDir};

/// Directories used during `peace` execution.
///
/// This type itself is not inserted into `Resources`, but each of the member
/// directories are individually inserted. This is created by
/// `CmdDirsBuilder` from either the `peace_rt_model` or
/// `peace_rt_model_web` crates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CmdDirs {
    /// Directory to store data for the current profile.
    profile_dir: ProfileDir,
    /// Directory to store profile executions' summaries.
    profile_history_dir: ProfileHistoryDir,
    /// Directory to store data for the current flow.
    flow_dir: FlowDir,
}

impl CmdDirs {
    /// Returns new `CmdDirs`.
    pub fn new(
        profile_dir: ProfileDir,
        profile_history_dir: ProfileHistoryDir,
        flow_dir: FlowDir,
    ) -> Self {
        Self {
            profile_dir,
            profile_history_dir,
            flow_dir,
        }
    }

    /// Returns the individual command directories.
    pub fn into_inner(self) -> (ProfileDir, ProfileHistoryDir, FlowDir) {
        let Self {
            profile_dir,
            profile_history_dir,
            flow_dir,
        } = self;

        (profile_dir, profile_history_dir, flow_dir)
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
