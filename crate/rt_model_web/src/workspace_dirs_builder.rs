use std::path::PathBuf;

use peace_core::AppName;
use peace_resources_rt::{
    internal::WorkspaceDirs,
    paths::{PeaceAppDir, PeaceDir},
};
use peace_rt_model_core::Error;

use crate::WorkspaceSpec;

/// Computes paths of well-known directories for a workspace.
#[derive(Debug)]
pub struct WorkspaceDirsBuilder;

impl WorkspaceDirsBuilder {
    /// Computes [`WorkspaceDirs`] paths.
    pub fn build(
        app_name: &AppName,
        workspace_spec: WorkspaceSpec,
    ) -> Result<WorkspaceDirs, Error> {
        // Written this way so that if we want to add a prefix, this would compile
        // error.
        let workspace_dir = match workspace_spec {
            WorkspaceSpec::LocalStorage | WorkspaceSpec::SessionStorage => PathBuf::from("").into(),
        };

        let peace_dir = PeaceDir::from(&workspace_dir);
        let peace_app_dir = PeaceAppDir::from((&peace_dir, app_name));

        Ok(WorkspaceDirs::new(workspace_dir, peace_dir, peace_app_dir))
    }
}
