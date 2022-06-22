use std::path::PathBuf;

use peace::{
    cfg::{async_trait, FullSpec},
    data::Resources,
};
use url::Url;

use crate::{
    DownloadCleanOpSpec, DownloadEnsureOpSpec, DownloadError, DownloadStatusFnSpec, FileState,
};

/// Full spec for downloading a file.
#[derive(Debug)]
pub struct DownloadFullSpec;

#[async_trait]
impl<'op> FullSpec<'op> for DownloadFullSpec {
    type CleanOpSpec = DownloadCleanOpSpec;
    type EnsureOpSpec = DownloadEnsureOpSpec;
    type Error = DownloadError;
    type StateLogical = Option<FileState>;
    type StatePhysical = PathBuf;
    type StatusFnSpec = DownloadStatusFnSpec;

    async fn setup(resources: &mut Resources) -> Result<(), DownloadError> {
        resources.insert::<reqwest::Client>(reqwest::Client::new());
        resources.insert::<Option<Url>>(None);
        resources.insert::<Option<PathBuf>>(None);

        Ok(())
    }
}
