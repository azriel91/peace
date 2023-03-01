use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, state::FetchedOpt, ItemSpec, ItemSpecId, State},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    ETag, FileDownloadCleanOpSpec, FileDownloadEnsureOpSpec, FileDownloadError, FileDownloadState,
    FileDownloadStateCurrentFnSpec, FileDownloadStateDesiredFnSpec, FileDownloadStateDiff,
    FileDownloadStateDiffFnSpec,
};

/// Item spec for downloading a file.
///
/// The `Id` type parameter is needed for each file download params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different file download
///   parameters from each other.
#[derive(Debug)]
pub struct FileDownloadItemSpec<Id> {
    /// ID of the item spec to download the file.
    item_spec_id: ItemSpecId,
    /// Marker for unique download parameters type.
    marker: PhantomData<Id>,
}

impl<Id> Clone for FileDownloadItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

impl<Id> FileDownloadItemSpec<Id> {
    /// Returns a new `FileDownloadItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for FileDownloadItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type CleanOpSpec = FileDownloadCleanOpSpec<Id>;
    type EnsureOpSpec = FileDownloadEnsureOpSpec<Id>;
    type Error = FileDownloadError;
    type State = State<FileDownloadState, FetchedOpt<ETag>>;
    type StateCurrentFnSpec = FileDownloadStateCurrentFnSpec<Id>;
    type StateDesiredFnSpec = FileDownloadStateDesiredFnSpec<Id>;
    type StateDiff = FileDownloadStateDiff;
    type StateDiffFnSpec = FileDownloadStateDiffFnSpec;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), FileDownloadError> {
        resources.insert::<reqwest::Client>(reqwest::Client::new());

        Ok(())
    }
}
