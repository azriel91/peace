//! Types that store information about the directories that a command runs in.
//!
//! In the Peace framework, a command is run with the following contextual
//! information:
//!
//! * The [`Workspace`] of a project that the command is built for.
//! * A [`Profile`] (or namespace) for that project.
//! * A workflow that the command is executing, identified by the [`FlowId`].
//!
//! # Implementors
//!
//! Sometimes, a command may manage information or items that are used in
//! different profiles, and as a framework consumer, the profile or flow ID is
//! not a "project environment profile" or "project workflow".
//!
//! In this case, [`Profile::workspace_init`], and [`FlowId::workspace_init`]
//! and [`FlowId::profile_init`] are defaulted by the [`WorkspaceBuilder`] for
//! the command's execution.

pub use self::workspace_builder::WorkspaceBuilder;

use peace_core::{AppName, FlowId, Profile};
use peace_resources::internal::WorkspaceDirs;
use peace_rt_model_core::workspace::ts::WorkspaceCommon;

use crate::{Error, Storage, WorkspaceSpec};

mod workspace_builder;

/// Workspace that the `peace` tool runs in.
#[derive(Clone, Debug)]
pub struct Workspace {
    /// Name of the application that is run by end users.
    app_name: AppName,
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
    /// If the command is not linked to a particular profile or flow, you may
    /// use [`Workspace::builder`] or [`WorkspaceBuilder::new`] to construct the
    /// `Workspace`.
    ///
    /// # Parameters
    ///
    /// * `app_name`: Name of the final application.
    /// * `workspace_spec`: Defines how to discover the workspace.
    /// * `profile`: The profile / namespace that the execution is flow.
    /// * `flow_id`: ID of the flow that is being executed.
    pub fn new(
        app_name: AppName,
        workspace_spec: WorkspaceSpec,
        profile: Profile,
        flow_id: FlowId,
    ) -> Result<Self, Error> {
        WorkspaceBuilder::new(app_name, workspace_spec)
            .with_profile(profile)
            .with_flow_id(flow_id)
            .build()
    }

    /// Returns the builder for a Workspace without setting a [`Profile`] or
    /// [`FlowId`].
    ///
    /// # Parameters
    ///
    /// * `app_name`: Name of the final application.
    /// * `workspace_spec`: Defines how to discover the workspace.
    pub fn builder(
        app_name: AppName,
        workspace_spec: WorkspaceSpec,
    ) -> WorkspaceBuilder<WorkspaceCommon> {
        WorkspaceBuilder::new(app_name, workspace_spec)
    }

    /// Returns the underlying data.
    pub fn into_inner(self) -> (AppName, WorkspaceDirs, Profile, FlowId, Storage) {
        let Self {
            app_name,
            dirs,
            profile,
            flow_id,
            storage,
        } = self;

        (app_name, dirs, profile, flow_id, storage)
    }

    /// Returns a reference to the app name.
    pub fn app_name(&self) -> &AppName {
        &self.app_name
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
