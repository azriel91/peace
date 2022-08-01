use std::path::PathBuf;

use diff::OptionDiff;
use peace::{
    cfg::{async_trait, full_spec_id, FullSpec, FullSpecId},
    resources::{resources_type_state::Empty, Resources},
};
use url::Url;

use crate::{
    DownloadCleanOpSpec, DownloadEnsureOpSpec, DownloadError, DownloadStateDesiredFnSpec,
    DownloadStateDiffFnSpec, DownloadStateNowFnSpec, FileState,
};

/// Full spec for downloading a file.
#[derive(Debug)]
pub struct DownloadFullSpec {
    /// Url of the file to download.
    src: Url,
    /// Path of the destination.
    ///
    /// Must be a file path, and not a directory.
    dest: PathBuf,
}

impl DownloadFullSpec {
    /// Returns a new FullSpec
    pub fn new(src: Url, dest: PathBuf) -> Self {
        Self { src, dest }
    }
}

#[async_trait]
impl FullSpec for DownloadFullSpec {
    type CleanOpSpec = DownloadCleanOpSpec;
    type EnsureOpSpec = DownloadEnsureOpSpec;
    type Error = DownloadError;
    type StateDesiredFnSpec = DownloadStateDesiredFnSpec;
    type StateDiff = OptionDiff<FileState>;
    type StateDiffFnSpec = DownloadStateDiffFnSpec;
    type StateLogical = Option<FileState>;
    type StateNowFnSpec = DownloadStateNowFnSpec;
    type StatePhysical = PathBuf;

    fn id(&self) -> FullSpecId {
        full_spec_id!("download_full_spec")
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), DownloadError> {
        resources.insert::<reqwest::Client>(reqwest::Client::new());
        resources.insert::<Url>(self.src.clone());
        resources.insert::<PathBuf>(self.dest.clone());

        Ok(())
    }
}
