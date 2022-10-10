#[nougat::gat(Data)]
use peace::cfg::CleanOpSpec;
use peace::cfg::{async_trait, nougat, state::Nothing, OpCheckStatus, ProgressLimit, State};

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
    type StateLogical = FileState;
    type StatePhysical = Nothing;

    async fn check(
        _download_params: DownloadParams<'_>,
        State {
            logical: file_state,
            ..
        }: &State<FileState, Nothing>,
    ) -> Result<OpCheckStatus, DownloadError> {
        let op_check_status = match file_state {
            FileState::None => OpCheckStatus::ExecNotRequired,
            FileState::StringContents { path: _, contents } => OpCheckStatus::ExecRequired {
                progress_limit: ProgressLimit::Bytes(contents.as_bytes().len().try_into().unwrap()),
            },
            FileState::Length {
                path: _,
                byte_count,
            } => OpCheckStatus::ExecRequired {
                progress_limit: ProgressLimit::Bytes(*byte_count),
            },
            FileState::Unknown { path: _ } => OpCheckStatus::ExecRequired {
                progress_limit: ProgressLimit::Unknown,
            },
        };
        Ok(op_check_status)
    }

    async fn exec_dry(
        _download_params: DownloadParams<'_>,
        _state: &State<FileState, Nothing>,
    ) -> Result<(), DownloadError> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        _download_params: DownloadParams<'_>,
        State {
            logical: file_state,
            ..
        }: &State<FileState, Nothing>,
    ) -> Result<(), DownloadError> {
        match file_state {
            FileState::None => {}
            FileState::StringContents { path, .. }
            | FileState::Length { path, .. }
            | FileState::Unknown { path } => {
                tokio::fs::remove_file(path)
                    .await
                    .map_err(DownloadError::DestFileRemove)?;
            }
        }
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    async fn exec(
        download_params: DownloadParams<'_>,
        State {
            logical: file_state,
            ..
        }: &State<FileState, Nothing>,
    ) -> Result<(), DownloadError> {
        match file_state {
            FileState::None => {}
            FileState::StringContents { path, .. }
            | FileState::Length { path, .. }
            | FileState::Unknown { path } => {
                download_params.storage().remove_item(path)?;
            }
        }

        Ok(())
    }
}
