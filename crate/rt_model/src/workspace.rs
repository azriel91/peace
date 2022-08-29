use std::{iter, path::Path};

#[cfg(not(target_arch = "wasm32"))]
use futures::{stream, StreamExt, TryStreamExt};
use peace_cfg::Profile;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

use crate::{Error, WorkspaceDirs, WorkspaceDirsBuilder, WorkspaceSpec};

/// Workspace that the `peace` tool runs in.
#[derive(Clone, Debug)]
pub struct Workspace {
    /// `Resources` in this workspace.
    dirs: WorkspaceDirs,
    /// Workspace profile used.
    profile: Profile,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(module = "/js/workspace.js")]
extern "C" {
    /// Returns whether local storage is available.
    fn localStorageAvailable() -> bool;
    /// Returns whether session storage is available.
    fn sessionStorageAvailable() -> bool;
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

        #[cfg(not(target_arch = "wasm32"))]
        Self::initialize_directories(&dirs).await?;

        #[cfg(target_arch = "wasm32")]
        Self::initialize_storage(workspace_spec, &dirs).await?;

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

    #[cfg(not(target_arch = "wasm32"))]
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

    #[cfg(target_arch = "wasm32")]
    async fn initialize_storage(
        workspace_spec: &WorkspaceSpec,
        dirs: &WorkspaceDirs,
    ) -> Result<(), Error> {
        let window = web_sys::window().ok_or(Error::WindowNone)?;
        let mut dirs = iter::once(AsRef::<Path>::as_ref(dirs.workspace_dir()))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.peace_dir())))
            .chain(iter::once(AsRef::<Path>::as_ref(dirs.profile_dir())))
            .chain(iter::once(AsRef::<Path>::as_ref(
                dirs.profile_history_dir(),
            )));

        let storage = match workspace_spec {
            WorkspaceSpec::LocalStorage => {
                if !localStorageAvailable() {
                    return Err(Error::LocalStorageUnavailable);
                }

                window
                    .local_storage()
                    .map_err(Self::stringify_js_value)
                    .map_err(Error::LocalStorageGet)?
                    .ok_or(Error::LocalStorageNone)?
            }
            WorkspaceSpec::SessionStorage => {
                if !sessionStorageAvailable() {
                    return Err(Error::SessionStorageUnavailable);
                }
                window
                    .session_storage()
                    .map_err(Self::stringify_js_value)
                    .map_err(Error::SessionStorageGet)?
                    .ok_or(Error::SessionStorageNone)?
            }
        };

        dirs.try_for_each(|dir| storage.set_item(&dir.to_string_lossy(), ""))
            .map_err(Self::stringify_js_value)
            .map_err(Error::StorageSetItem)
    }

    /// Converts the `JsValue` to a `String` to allow `Error` to be `Send`.
    #[cfg(target_arch = "wasm32")]
    fn stringify_js_value(js_value: JsValue) -> String {
        js_value
            .into_serde::<String>()
            .unwrap_or_else(|_| String::from("<??>"))
    }
}
