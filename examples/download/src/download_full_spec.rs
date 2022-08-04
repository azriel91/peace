use std::path::PathBuf;

use peace::{
    cfg::{async_trait, full_spec_id, FullSpec, FullSpecId},
    resources::{resources_type_state::Empty, Resources},
};
use url::Url;

use crate::{
    DownloadCleanOpSpec, DownloadEnsureOpSpec, DownloadError, DownloadStateCurrentFnSpec,
    DownloadStateDesiredFnSpec, DownloadStateDiffFnSpec, FileState, FileStateDiff,
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

#[async_trait(?Send)]
impl FullSpec for DownloadFullSpec {
    type CleanOpSpec = DownloadCleanOpSpec;
    type EnsureOpSpec = DownloadEnsureOpSpec;
    type Error = DownloadError;
    type StateCurrentFnSpec = DownloadStateCurrentFnSpec;
    type StateDesiredFnSpec = DownloadStateDesiredFnSpec;
    type StateDiff = FileStateDiff;
    type StateDiffFnSpec = DownloadStateDiffFnSpec;
    type StateLogical = Option<FileState>;
    type StatePhysical = PathBuf;

    fn id(&self) -> FullSpecId {
        full_spec_id!("download_full_spec")
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), DownloadError> {
        resources.insert::<reqwest::Client>(reqwest::Client::new());
        resources.insert::<Url>(self.src.clone());
        resources.insert::<PathBuf>(self.dest.clone());

        #[cfg(target_arch = "wasm32")]
        resources.insert(std::collections::HashMap::<PathBuf, String>::new());

        Ok(())
    }
}
