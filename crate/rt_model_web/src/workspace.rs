use std::{iter, path::Path};

use peace_core::{FlowId, Profile};
use peace_resources::internal::WorkspaceDirs;

use crate::{Error, WebStorage, WorkspaceDirsBuilder, WorkspaceSpec};

/// Workspace that the `peace` tool runs in.
#[derive(Clone, Debug)]
pub struct Workspace {
    /// `Resources` in this workspace.
    dirs: WorkspaceDirs,
    /// Identifier or namespace to distinguish execution environments.
    profile: Profile,
    /// Identifier or name of the chosen process flow.
    flow_id: FlowId,
    /// Wrapper to retrieve `web_sys::Storage` on demand.
    storage: WebStorage,
}

impl Workspace {
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace_spec`: Defines how to discover the workspace.
    /// * `profile`: The profile / namespace that the execution is flow.
    /// * `flow_id`: ID of the flow that is being executed.
    pub async fn init(
        workspace_spec: WorkspaceSpec,
        profile: Profile,
        flow_id: FlowId,
    ) -> Result<Workspace, Error> {
        let dirs = WorkspaceDirsBuilder::build(workspace_spec, &profile, &flow_id)?;
        let storage = Self::initialize_storage(workspace_spec, &dirs).await?;

        Ok(Workspace {
            dirs,
            profile,
            flow_id,
            storage,
        })
    }

    /// Returns the inner data.
    pub fn into_inner(self) -> (WorkspaceDirs, Profile, FlowId, WebStorage) {
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

    /// Returns a reference to the workspace's flow ID.
    pub fn flow_id(&self) -> &FlowId {
        &self.flow_id
    }

    /// Returns the storage used for this workspace.
    pub fn storage(&self) -> &WebStorage {
        &self.storage
    }

    async fn initialize_storage(
        workspace_spec: WorkspaceSpec,
        dirs: &WorkspaceDirs,
    ) -> Result<WebStorage, Error> {
        let dirs = iter::once(AsRef::<Path>::as_ref(dirs.workspace_dir()))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.peace_dir())))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.profile_dir())))
            .chain(iter::once(AsRef::<Path>::as_ref(
                dirs.profile_history_dir(),
            )))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.flow_dir())));

        let workspace_storage = WebStorage::new(workspace_spec);
        workspace_storage.iter_with_storage(dirs, |storage, dir| {
            let dir_str = dir.to_string_lossy();
            let value = "";
            storage
                .set_item(dir_str.as_ref(), value)
                .map_err(|js_value| (dir_str.to_string(), "".to_string(), js_value))
        })?;

        Ok(workspace_storage)
    }
}
