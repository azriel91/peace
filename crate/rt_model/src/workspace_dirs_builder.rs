use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use std::{ffi::OsStr, path::Path};

use peace_core::Profile;
use peace_resources::{
    dir::{PeaceDir, ProfileDir, ProfileHistoryDir},
    internal::WorkspaceDirs,
};

use crate::{Error, WorkspaceSpec};

/// Computes paths of well-known directories for a workspace.
#[derive(Debug)]
pub struct WorkspaceDirsBuilder;

impl WorkspaceDirsBuilder {
    /// Computes [`WorkspaceDirs`] paths.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build(
        workspace_spec: &WorkspaceSpec,
        profile: &Profile,
    ) -> Result<WorkspaceDirs, Error> {
        use peace_resources::dir::WorkspaceDir;

        let workspace_dir = {
            let working_dir = std::env::current_dir().map_err(Error::WorkingDirRead)?;
            let workspace_dir = match workspace_spec {
                WorkspaceSpec::WorkingDir => working_dir,
                WorkspaceSpec::Path(path) => path.clone(),
                WorkspaceSpec::FirstDirWithFile(file_name) => {
                    Self::first_dir_with_file(&working_dir, file_name).ok_or_else(move || {
                        let file_name = file_name.to_os_string();
                        Error::WorkspaceFileNotFound {
                            working_dir,
                            file_name,
                        }
                    })?
                }
            };

            WorkspaceDir::new(workspace_dir)
        };

        let peace_dir = PeaceDir::from(&workspace_dir);
        let profile_dir = ProfileDir::from((&peace_dir, profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        Ok(WorkspaceDirs::new(
            workspace_dir,
            peace_dir,
            profile_dir,
            profile_history_dir,
        ))
    }

    /// Computes [`WorkspaceDirs`] paths.
    #[cfg(target_arch = "wasm32")]
    pub fn build(
        web_storage_spec: &peace_web_support::WebStorageSpec,
        profile: &Profile,
    ) -> Result<WorkspaceDirs, Error> {
        use peace_web_support::WebStorageSpec;

        // Written this way so that if we want to add a prefix, this would compile
        // error.
        let workspace_dir = match web_storage_spec {
            WebStorageSpec::LocalStorage | WebStorageSpec::SessionStorage => {
                PathBuf::from("/").into()
            }
        };

        let peace_dir = PeaceDir::from(&workspace_dir);
        let profile_dir = ProfileDir::from((&peace_dir, profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        Ok(WorkspaceDirs::new(
            workspace_dir,
            peace_dir,
            profile_dir,
            profile_history_dir,
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn first_dir_with_file(working_dir: &Path, path: &OsStr) -> Option<PathBuf> {
        let mut candidate_dir = working_dir.to_path_buf();
        loop {
            let candidate_marker = candidate_dir.join(path);
            if candidate_marker.exists() {
                return Some(candidate_dir);
            }

            // pop() returns false if there is no parent dir.
            if !candidate_dir.pop() {
                return None;
            }
        }
    }
}
