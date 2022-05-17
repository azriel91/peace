use std::path::PathBuf;

use zzzz::cfg::{async_trait::async_trait, OpCheckStatus, OpSpec, OpSpecDry, ProgressLimit};

use crate::{DownloadError, DownloadParams, FileState};

/// Clean OpSpec for the file to download.
#[derive(Debug)]
pub struct DownloadCleanSpec;

#[async_trait]
impl OpSpec for DownloadCleanSpec {
    type Data = DownloadParams;
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
        tokio::fs::remove_file(&download_params.dest())
            .await
            .map_err(DownloadError::DestFileRemove)?;
        Ok(download_params.dest().to_path_buf())
    }
}

#[async_trait]
impl OpSpecDry for DownloadCleanSpec {
    async fn exec_dry() -> Result<Self::Output, Self::Error> {
        todo!("should this be inferred from the Diff instead")
    }
}
