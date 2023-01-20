use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use peace_core::AppName;
use peace_resources::{
    internal::WorkspaceDirs,
    paths::{PeaceAppDir, PeaceDir},
};

use crate::{Error, WorkspaceSpec};

/// Computes paths of well-known directories for a workspace.
#[derive(Debug)]
pub struct WorkspaceDirsBuilder;

impl WorkspaceDirsBuilder {
    /// Computes [`WorkspaceDirs`] paths.
    pub fn build(
        app_name: &AppName,
        workspace_spec: WorkspaceSpec,
    ) -> Result<WorkspaceDirs, Error> {
        use peace_resources::paths::WorkspaceDir;

        let workspace_dir = {
            let working_dir = std::env::current_dir().map_err(Error::WorkingDirRead)?;
            let workspace_dir = match workspace_spec {
                WorkspaceSpec::WorkingDir => working_dir,
                WorkspaceSpec::Path(path) => path,
                WorkspaceSpec::FirstDirWithFile(file_name) => {
                    Self::first_dir_with_file(&working_dir, &file_name).ok_or({
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
        let peace_app_dir = PeaceAppDir::from((&peace_dir, app_name));

        Ok(WorkspaceDirs::new(workspace_dir, peace_dir, peace_app_dir))
    }

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
