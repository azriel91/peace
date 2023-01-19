use std::path::PathBuf;

use peace_core::{AppName, FlowId, Profile};
use peace_resources::{
    internal::WorkspaceDirs,
    paths::{FlowDir, PeaceAppDir, PeaceDir, ProfileDir, ProfileHistoryDir},
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
        profile: &Profile,
        flow_id: &FlowId,
    ) -> Result<WorkspaceDirs, Error> {
        // Written this way so that if we want to add a prefix, this would compile
        // error.
        let workspace_dir = match workspace_spec {
            WorkspaceSpec::LocalStorage | WorkspaceSpec::SessionStorage => PathBuf::from("").into(),
        };

        let peace_dir = PeaceDir::from(&workspace_dir);
        let peace_app_dir = PeaceAppDir::from((&peace_dir, app_name));
        let profile_dir = ProfileDir::from((&peace_app_dir, profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let flow_dir = FlowDir::from((&profile_dir, flow_id));

        Ok(WorkspaceDirs::new(
            workspace_dir,
            peace_dir,
            peace_app_dir,
            profile_dir,
            profile_history_dir,
            flow_dir,
        ))
    }
}
