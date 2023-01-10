use peace_core::{FlowId, Profile};
use peace_resources::internal::WorkspaceDirs;

use crate::{Error, Storage, WorkspaceDirsBuilder, WorkspaceSpec};

/// Workspace that the `peace` tool runs in.
#[derive(Clone, Debug)]
pub struct Workspace {
    /// Convention-based directories in this workspace.
    dirs: WorkspaceDirs,
    /// Identifier or namespace to distinguish execution environments.
    profile: Profile,
    /// Identifier or name of the chosen process flow.
    flow_id: FlowId,
    /// File system storage access.
    storage: Storage,
}

impl Workspace {
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace_spec`: Defines how to discover the workspace.
    /// * `profile`: The profile / namespace that the execution is flow.
    /// * `flow_id`: ID of the flow that is being executed.
    pub fn new(
        workspace_spec: WorkspaceSpec,
        profile: Profile,
        flow_id: FlowId,
    ) -> Result<Self, Error> {
        let dirs = WorkspaceDirsBuilder::build(workspace_spec, &profile, &flow_id)?;
        let storage = Storage;

        Ok(Self {
            dirs,
            profile,
            flow_id,
            storage,
        })
    }

    /// Returns the underlying data.
    pub fn into_inner(self) -> (WorkspaceDirs, Profile, FlowId, Storage) {
        let Self {
            dirs,
            profile,
            flow_id,
            storage,
        } = self;

        (dirs, profile, flow_id, storage)
    }

    /// Returns a reference to the workspace's directories.
    pub fn dirs(&self) -> &WorkspaceDirs {
        &self.dirs
    }

    /// Returns a reference to the workspace's profile.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// Returns a reference to the workspace's flow_id.
    pub fn flow_id(&self) -> &FlowId {
        &self.flow_id
    }

    /// Returns a reference to the workspace's storage.
    pub fn storage(&self) -> &Storage {
        &self.storage
    }
}
