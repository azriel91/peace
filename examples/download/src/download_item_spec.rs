use std::path::PathBuf;

use peace::{
    cfg::{async_trait, item_spec_id, ItemSpec, ItemSpecId},
    resources::{resources_type_state::Empty, Resources},
};

use crate::{
    DownloadCleanOpSpec, DownloadEnsureOpSpec, DownloadError, DownloadStateCurrentFnSpec,
    DownloadStateDesiredFnSpec, DownloadStateDiffFnSpec, FileState, FileStateDiff,
};

/// Full spec for downloading a file.
#[derive(Debug)]
pub struct DownloadItemSpec;

#[async_trait(?Send)]
impl ItemSpec for DownloadItemSpec {
    type CleanOpSpec = DownloadCleanOpSpec;
    type EnsureOpSpec = DownloadEnsureOpSpec;
    type Error = DownloadError;
    type StateCurrentFnSpec = DownloadStateCurrentFnSpec;
    type StateDesiredFnSpec = DownloadStateDesiredFnSpec;
    type StateDiff = FileStateDiff;
    type StateDiffFnSpec = DownloadStateDiffFnSpec;
    type StateLogical = Option<FileState>;
    type StatePhysical = PathBuf;

    fn id(&self) -> ItemSpecId {
        item_spec_id!("download")
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), DownloadError> {
        resources.insert::<reqwest::Client>(reqwest::Client::new());

        #[cfg(target_arch = "wasm32")]
        resources.insert(std::collections::HashMap::<PathBuf, String>::new());

        Ok(())
    }
}
