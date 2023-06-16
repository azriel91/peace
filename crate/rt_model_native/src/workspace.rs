//! Types that store information about the directories that a command runs in.
//!
//! In the Peace framework, a command is run with the following contextual
//! information:
//!
//! * The [`Workspace`] of a project that the command is built for.
//! * A [`Profile`] (or namespace) for that project.
//! * A workflow that the command is executing, identified by the [`FlowId`].

use peace_core::AppName;
use peace_resources::internal::WorkspaceDirs;
use peace_rt_model_core::Error;

use crate::{Storage, WorkspaceDirsBuilder, WorkspaceSpec};

/// Workspace that the `peace` tool runs in.
#[derive(Clone, Debug)]
pub struct Workspace {
    /// Name of the application that is run by end users.
    app_name: AppName,
    /// Convention-based directories in this workspace.
    dirs: WorkspaceDirs,
    /// File system storage access.
    storage: Storage,
}

impl Workspace {
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `app_name`: Name of the final application.
    /// * `workspace_spec`: Defines how to discover the workspace.
    pub fn new(app_name: AppName, workspace_spec: WorkspaceSpec) -> Result<Self, Error> {
        let dirs = WorkspaceDirsBuilder::build(&app_name, workspace_spec)?;
        let storage = Storage;

        Ok(Self {
            app_name,
            dirs,
            storage,
        })
    }

    /// Returns the underlying data.
    pub fn into_inner(self) -> (AppName, WorkspaceDirs, Storage) {
        let Self {
            app_name,
            dirs,
            storage,
        } = self;

        (app_name, dirs, storage)
    }

    /// Returns a reference to the app name.
    pub fn app_name(&self) -> &AppName {
        &self.app_name
    }

    /// Returns a reference to the workspace's directories.
    pub fn dirs(&self) -> &WorkspaceDirs {
        &self.dirs
    }

    /// Returns a reference to the workspace's storage.
    pub fn storage(&self) -> &Storage {
        &self.storage
    }
}
