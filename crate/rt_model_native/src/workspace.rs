use std::{iter, path::Path};

use futures::{stream, StreamExt, TryStreamExt};
use peace_core::Profile;
use peace_resources::internal::WorkspaceDirs;

use crate::{Error, NativeStorage, WorkspaceDirsBuilder, WorkspaceSpec};

/// Workspace that the `peace` tool runs in.
#[derive(Clone, Debug)]
pub struct Workspace {
    /// `Resources` in this workspace.
    dirs: WorkspaceDirs,
    /// Workspace profile used.
    profile: Profile,
    /// File system storage access.
    storage: NativeStorage,
}

impl Workspace {
    /// Prepares a workspace to run commands in.
    ///
    /// # Parameters
    ///
    /// * `workspace_spec`: Defines how to discover the workspace.
    /// * `profile`: The profile that execution is .
    pub async fn init(workspace_spec: WorkspaceSpec, profile: Profile) -> Result<Workspace, Error> {
        let dirs = WorkspaceDirsBuilder::build(workspace_spec, &profile)?;
        Self::initialize_directories(&dirs).await?;

        let storage = NativeStorage;
        Ok(Workspace {
            dirs,
            profile,
            storage,
        })
    }

    /// Returns the inner data.
    pub fn into_inner(self) -> (WorkspaceDirs, Profile, NativeStorage) {
        let Self {
            dirs,
            profile,
            storage,
        } = self;

        (dirs, profile, storage)
    }

    /// Returns a reference to the workspace's directories.
    pub fn dirs(&self) -> &WorkspaceDirs {
        &self.dirs
    }

    /// Returns a reference to the workspace's profile.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// Returns a reference to the workspace's storage.
    pub fn storage(&self) -> &NativeStorage {
        &self.storage
    }

    async fn initialize_directories(dirs: &WorkspaceDirs) -> Result<(), Error> {
        let dirs = iter::once(AsRef::<Path>::as_ref(dirs.workspace_dir()))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.peace_dir())))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.profile_dir())))
            .chain(iter::once(AsRef::<Path>::as_ref(
                dirs.profile_history_dir(),
            )));

        stream::iter(dirs)
            .map(Result::<_, Error>::Ok)
            .try_for_each(|dir| async move {
                tokio::fs::create_dir_all(dir).await.map_err(|error| {
                    let path = dir.to_path_buf();
                    Error::WorkspaceDirCreate { path, error }
                })
            })
            .await
    }
}
