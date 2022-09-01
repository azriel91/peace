#[cfg(not(target_arch = "wasm32"))]
pub use self::native::Workspace;

#[cfg(target_arch = "wasm32")]
pub use self::wasm::Workspace;

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::{iter, path::Path};

    use futures::{stream, StreamExt, TryStreamExt};
    use peace_cfg::Profile;
    use peace_resources::internal::WorkspaceDirs;

    use crate::{Error, WorkspaceDirsBuilder, WorkspaceSpec};

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
        pub async fn init(
            workspace_spec: &WorkspaceSpec,
            profile: Profile,
        ) -> Result<Workspace, Error> {
            let dirs = WorkspaceDirsBuilder::build(workspace_spec, &profile)?;
            Self::initialize_directories(&dirs).await?;
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
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use std::{iter, path::Path};

    use peace_cfg::Profile;
    use peace_resources::internal::WorkspaceDirs;
    use peace_web_support::{Error, WebStorage, WebStorageSpec, WorkspaceDirsBuilder};

    /// Workspace that the `peace` tool runs in.
    #[derive(Clone, Debug)]
    pub struct Workspace {
        /// `Resources` in this workspace.
        dirs: WorkspaceDirs,
        /// Workspace profile used.
        profile: Profile,
        /// Wrapper to retrieve `web_sys::Storage` on demand.
        storage: WebStorage,
    }

    impl Workspace {
        /// Prepares a workspace to run commands in.
        ///
        /// # Parameters
        ///
        /// * `workspace_spec`: Defines how to discover the workspace.
        /// * `profile`: The profile that execution is .
        pub async fn init(
            web_storage_spec: WebStorageSpec,
            profile: Profile,
        ) -> Result<Workspace, Error> {
            let dirs = WorkspaceDirsBuilder::build(web_storage_spec, &profile)?;
            let storage = Self::initialize_storage(web_storage_spec, &dirs).await?;

            Ok(Workspace {
                dirs,
                profile,
                storage,
            })
        }

        /// Returns the inner data.
        pub fn into_inner(self) -> (WorkspaceDirs, Profile, WebStorage) {
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

        /// Returns the storage used for this workspace.
        pub fn storage(&self) -> &WebStorage {
            &self.storage
        }

        async fn initialize_storage(
            web_storage_spec: WebStorageSpec,
            dirs: &WorkspaceDirs,
        ) -> Result<WebStorage, Error> {
            let dirs = iter::once(AsRef::<Path>::as_ref(dirs.workspace_dir()))
                .chain(iter::once(AsRef::<Path>::as_ref(dirs.peace_dir())))
                .chain(iter::once(AsRef::<Path>::as_ref(dirs.profile_dir())))
                .chain(iter::once(AsRef::<Path>::as_ref(
                    dirs.profile_history_dir(),
                )));

            let workspace_storage = WebStorage::new(web_storage_spec);
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
}
