use peace::{
    cfg::{async_trait, state::Nothing, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    FileDownloadCleanOpSpec, FileDownloadEnsureOpSpec, FileDownloadError, FileDownloadState,
    FileDownloadStateCurrentFnSpec, FileDownloadStateDesiredFnSpec, FileDownloadStateDiff,
    FileDownloadStateDiffFnSpec,
};

/// Item spec for downloading a file.
#[derive(Debug)]
pub struct FileDownloadItemSpec {
    /// ID of the item spec to download the file.
    item_spec_id: ItemSpecId,
}

impl FileDownloadItemSpec {
    /// Returns a new `FileDownloadItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self { item_spec_id }
    }
}

#[async_trait(?Send)]
impl ItemSpec for FileDownloadItemSpec {
    type CleanOpSpec = FileDownloadCleanOpSpec;
    type EnsureOpSpec = FileDownloadEnsureOpSpec;
    type Error = FileDownloadError;
    type StateCurrentFnSpec = FileDownloadStateCurrentFnSpec;
    type StateDesiredFnSpec = FileDownloadStateDesiredFnSpec;
    type StateDiff = FileDownloadStateDiff;
    type StateDiffFnSpec = FileDownloadStateDiffFnSpec;
    type StateLogical = FileDownloadState;
    type StatePhysical = Nothing;

    fn id(&self) -> ItemSpecId {
        self.item_spec_id.clone()
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), FileDownloadError> {
        resources.insert::<reqwest::Client>(reqwest::Client::new());

        Ok(())
    }
}
