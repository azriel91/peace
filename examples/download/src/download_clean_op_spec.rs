use std::path::PathBuf;

#[nougat::gat(Data)]
use peace::cfg::CleanOpSpec;
use peace::cfg::{async_trait, nougat, OpCheckStatus, ProgressLimit, State};

use crate::{DownloadError, DownloadParams, FileState};

/// `CleanOpSpec` for the file to download.
#[derive(Debug)]
pub struct DownloadCleanOpSpec;

#[async_trait(?Send)]
#[nougat::gat]
impl CleanOpSpec for DownloadCleanOpSpec {
    type Data<'op> = DownloadParams<'op>
        where Self: 'op;
    type Error = DownloadError;
    type StateLogical = Option<FileState>;
    type StatePhysical = PathBuf;

    #[cfg(not(target_arch = "wasm32"))]
    async fn check(
        _download_params: DownloadParams<'_>,
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

    #[cfg(target_arch = "wasm32")]
    async fn check(
        download_params: DownloadParams<'_>,
        State {
            physical: dest_path,
            ..
        }: &State<Option<FileState>, PathBuf>,
    ) -> Result<OpCheckStatus, DownloadError> {
        let dest_path_exists = download_params
            .storage()
            .get_item_opt(&dest_path)?
            .is_some();
        let op_check_status = if dest_path_exists {
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
        _download_params: DownloadParams<'_>,
        _state: &State<Option<FileState>, PathBuf>,
    ) -> Result<(), DownloadError> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        _download_params: DownloadParams<'_>,
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

    #[cfg(target_arch = "wasm32")]
    async fn exec(
        download_params: DownloadParams<'_>,
        State {
            physical: dest_path,
            ..
        }: &State<Option<FileState>, PathBuf>,
    ) -> Result<(), DownloadError> {
        download_params.storage().remove_item(dest_path)?;

        Ok(())
    }
}
