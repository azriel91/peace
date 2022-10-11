use peace::{
    cfg::{async_trait, state::Nothing, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    DownloadCleanOpSpec, DownloadEnsureOpSpec, DownloadError, DownloadStateCurrentFnSpec,
    DownloadStateDesiredFnSpec, DownloadStateDiffFnSpec, FileState, FileStateDiff,
};

/// Item spec for downloading a file.
#[derive(Debug)]
pub struct FileItemSpec {
    /// ID of the item spec to download the file.
    item_spec_id: ItemSpecId,
}

impl FileItemSpec {
    /// Returns a new `FileItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self { item_spec_id }
    }
}

#[async_trait(?Send)]
impl ItemSpec for FileItemSpec {
    type CleanOpSpec = DownloadCleanOpSpec;
    type EnsureOpSpec = DownloadEnsureOpSpec;
    type Error = DownloadError;
    type StateCurrentFnSpec = DownloadStateCurrentFnSpec;
    type StateDesiredFnSpec = DownloadStateDesiredFnSpec;
    type StateDiff = FileStateDiff;
    type StateDiffFnSpec = DownloadStateDiffFnSpec;
    type StateLogical = FileState;
    type StatePhysical = Nothing;

    fn id(&self) -> ItemSpecId {
        self.item_spec_id.clone()
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), DownloadError> {
        resources.insert::<reqwest::Client>(reqwest::Client::new());

        Ok(())
    }
}
