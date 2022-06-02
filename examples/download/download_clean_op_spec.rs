use std::path::PathBuf;

use peace::cfg::{async_trait, OpCheckStatus, OpSpec, OpSpecDry, ProgressLimit};

use crate::{DownloadError, DownloadParams, FileState};

/// Clean OpSpec for the file to download.
#[derive(Debug)]
pub struct DownloadCleanOpSpec;

#[async_trait]
impl<'op> OpSpec<'op> for DownloadCleanOpSpec {
    type Data = DownloadParams<'op>;
    type Error = DownloadError;
    type Output = PathBuf;
    type State = Option<FileState>;

    async fn setup(_download_params: &DownloadParams) -> Result<ProgressLimit, DownloadError> {
        // TODO: pass through desired State,

        // Bytes to delete
        Ok(ProgressLimit::Bytes(1024))
    }

    async fn check(
        _download_params: &DownloadParams,
        file_state: &Option<FileState>,
    ) -> Result<OpCheckStatus, DownloadError> {
        let op_check_status = if file_state.is_some() {
            OpCheckStatus::ExecRequired
        } else {
            OpCheckStatus::ExecNotRequired
        };
        Ok(op_check_status)
    }

    async fn exec(download_params: &DownloadParams) -> Result<PathBuf, DownloadError> {
        let dest = download_params.dest().ok_or(DownloadError::DestFileInit)?;
        tokio::fs::remove_file(dest)
            .await
            .map_err(DownloadError::DestFileRemove)?;
        Ok(dest.to_path_buf())
    }
}

#[async_trait]
impl<'op> OpSpecDry<'op> for DownloadCleanOpSpec {
    async fn exec_dry() -> Result<Self::Output, Self::Error> {
        todo!("should this be inferred from the Diff instead")
    }
}
