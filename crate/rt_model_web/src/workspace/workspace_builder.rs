use std::marker::PhantomData;

use peace_core::{FlowId, Profile};
use peace_rt_model_core::workspace::ts::{FlowIdSelected, ProfileSelected, WorkspaceCommon};

use crate::{workspace::Workspace, Error, Storage, WorkspaceDirsBuilder, WorkspaceSpec};

/// Workspace that the commands run in.
#[derive(Clone, Debug)]
pub struct WorkspaceBuilder<TS> {
    /// Describes how to discover the workspace directory.
    workspace_spec: WorkspaceSpec,
    /// Identifier or namespace to distinguish execution environments.
    profile: Profile,
    /// Identifier or name of the chosen process flow.
    flow_id: FlowId,
    /// Marker.
    marker: PhantomData<TS>,
}

impl WorkspaceBuilder<WorkspaceCommon> {
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace_spec`: Defines how to discover the workspace.
    pub fn new(workspace_spec: WorkspaceSpec) -> Self {
        Self {
            workspace_spec,
            profile: Profile::workspace_init(),
            flow_id: FlowId::workspace_init(),
            marker: PhantomData,
        }
    }

    /// Selects the profile for the workspace.
    ///
    /// If this is not called, then the [`"default"`] profile is used.
    ///
    /// [`"default"`]: Profile::default
    pub fn with_profile(self, profile: Profile) -> WorkspaceBuilder<ProfileSelected> {
        let WorkspaceBuilder {
            workspace_spec,
            profile: _,
            flow_id: _,
            marker: _,
        } = self;

        WorkspaceBuilder {
            workspace_spec,
            profile,
            flow_id: FlowId::profile_init(),
            marker: PhantomData,
        }
    }
}

impl WorkspaceBuilder<ProfileSelected> {
    /// Selects the flow ID for the workspace.
    ///
    /// If this is not called, then the [`"default"`] flow ID is used.
    ///
    /// [`"default"`]: FlowId::default
    pub fn with_flow_id(self, flow_id: FlowId) -> WorkspaceBuilder<FlowIdSelected> {
        let WorkspaceBuilder {
            workspace_spec,
            profile,
            flow_id: _,
            marker: _,
        } = self;

        WorkspaceBuilder {
            workspace_spec,
            profile,
            flow_id,
            marker: PhantomData,
        }
    }
}

impl<TS> WorkspaceBuilder<TS> {
    /// Builds and returns the `Workspace`.
    pub fn build(self) -> Result<Workspace, Error> {
        let WorkspaceBuilder {
            workspace_spec,
            profile,
            flow_id,
            marker: _,
        } = self;

        let dirs = WorkspaceDirsBuilder::build(workspace_spec, &profile, &flow_id)?;
        let storage = Storage::new(workspace_spec);

        Ok(Workspace {
            dirs,
            profile,
            flow_id,
            storage,
        })
    }
}