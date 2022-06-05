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
#[derive(Debug, Default)]
pub struct DownloadFullSpec {
    status_fn_spec: DownloadStatusFnSpec,
    ensure_op_spec: DownloadEnsureOpSpec,
    clean_op_spec: DownloadCleanOpSpec,
}

#[async_trait]
impl<'op> FullSpec<'op> for DownloadFullSpec {
    type CleanOpSpec = DownloadCleanOpSpec;
    type EnsureOpSpec = DownloadEnsureOpSpec;
    type Error = DownloadError;
    type ResIds = PathBuf;
    type State = Option<FileState>;
    type StatusFnSpec = DownloadStatusFnSpec;

    fn status_fn_spec(&self) -> &Self::StatusFnSpec {
        &self.status_fn_spec
    }

    fn ensure_op_spec(&self) -> &Self::EnsureOpSpec {
        &self.ensure_op_spec
    }

    fn clean_op_spec(&self) -> &Self::CleanOpSpec {
        &self.clean_op_spec
    }

    async fn setup(resources: &mut Resources) -> Result<(), DownloadError> {
        resources.insert::<reqwest::Client>(reqwest::Client::new());
        resources.insert::<Option<Url>>(None);
        resources.insert::<Option<PathBuf>>(None);

        Ok(())
    }
}
