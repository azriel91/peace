use std::path::PathBuf;

use peace_core::{FlowId, Profile};
use peace_resources::{
    dir::{FlowDir, PeaceDir, ProfileDir, ProfileHistoryDir},
    internal::WorkspaceDirs,
};

use crate::{Error, WorkspaceSpec};

/// Computes paths of well-known directories for a workspace.
#[derive(Debug)]
pub struct WorkspaceDirsBuilder;

impl WorkspaceDirsBuilder {
    /// Computes [`WorkspaceDirs`] paths.
    pub fn build(
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
        let profile_dir = ProfileDir::from((&peace_dir, profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let flow_dir = FlowDir::from((&profile_dir, flow_id));

        Ok(WorkspaceDirs::new(
            workspace_dir,
            peace_dir,
            profile_dir,
            profile_history_dir,
            flow_dir,
        ))
    }
}
