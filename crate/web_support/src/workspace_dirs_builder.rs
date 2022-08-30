use std::path::PathBuf;

use peace_core::Profile;
use peace_resources::{
    dir::{PeaceDir, ProfileDir, ProfileHistoryDir},
    internal::WorkspaceDirs,
};

use crate::{Error, WebStorageSpec};

/// Computes paths of well-known directories for a workspace.
#[derive(Debug)]
pub struct WorkspaceDirsBuilder;

impl WorkspaceDirsBuilder {
    /// Computes [`WorkspaceDirs`] paths.
    pub fn build(
        web_storage_spec: WebStorageSpec,
        profile: &Profile,
    ) -> Result<WorkspaceDirs, Error> {
        // Written this way so that if we want to add a prefix, this would compile
        // error.
        let workspace_dir = match web_storage_spec {
            WebStorageSpec::LocalStorage | WebStorageSpec::SessionStorage => {
                PathBuf::from("").into()
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
}
