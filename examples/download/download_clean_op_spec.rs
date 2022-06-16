use std::path::PathBuf;

use peace::cfg::{async_trait, CleanOpSpec, OpCheckStatus, ProgressLimit, State};

use crate::{DownloadError, DownloadParams, FileState};

/// `CleanOpSpec` for the file to download.
#[derive(Debug)]
pub struct DownloadCleanOpSpec;

#[async_trait]
impl<'op> CleanOpSpec<'op> for DownloadCleanOpSpec {
    type Data = DownloadParams<'op>;
    type Error = DownloadError;
    type StateLogical = Option<FileState>;
    type StatePhysical = PathBuf;

    async fn check(
        _download_params: DownloadParams<'op>,
        State {
            physical: dest_path,
            ..
        }: &State<Option<FileState>, PathBuf>,
    ) -> Result<OpCheckStatus, DownloadError> {
        let op_check_status = if dest_path.exists() {
            // TODO: read file size
            OpCheckStatus::ExecRequired {
                progress_limit: ProgressLimit::Bytes(1024),
            }
        } else {
            OpCheckStatus::ExecNotRequired
        };
        Ok(op_check_status)
    }

    async fn exec_dry(
        _download_params: DownloadParams<'op>,
        _state: &State<Option<FileState>, PathBuf>,
    ) -> Result<(), DownloadError> {
        Ok(())
    }

    async fn exec(
        _download_params: DownloadParams<'op>,
        State {
            physical: dest_path,
            ..
        }: &State<Option<FileState>, PathBuf>,
    ) -> Result<(), DownloadError> {
        tokio::fs::remove_file(dest_path)
            .await
            .map_err(DownloadError::DestFileRemove)?;
        Ok(())
    }
}
