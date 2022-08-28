use peace_cfg::Profile;

use crate::{Error, WorkspaceDirs, WorkspaceDirsBuilder, WorkspaceSpec};

/// Workspace that the `peace` tool runs in.
#[derive(Clone, Debug)]
pub struct Workspace {
    /// `Resources` in this workspace.
    dirs: WorkspaceDirs,
    /// Workspace profile used.
    profile: Profile,
}

impl Workspace {
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace_spec`: Defines how to discover the workspace.
    /// * `profile`: The profile that execution is .
    pub async fn try_new(
        workspace_spec: &WorkspaceSpec,
        profile: Profile,
    ) -> Result<Workspace, Error> {
        let dirs = WorkspaceDirsBuilder::build(workspace_spec, &profile)?;

        Ok(Workspace { dirs, profile })
    }

    /// Returns the inner data.
    pub fn into_inner(self) -> (WorkspaceDirs, Profile) {
        let Self { dirs, profile } = self;

        (dirs, profile)
    }

    /// Returns a reference to the workspace's directories.
    pub fn dirs(&self) -> &WorkspaceDirs {
        &self.dirs
    }

    /// Returns a reference to the workspace's profile.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }
}
